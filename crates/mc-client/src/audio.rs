//! Real audio for the generated client: official menu music decoded with
//! symphonia and played through pluggable output backends.
//!
//! Generator-first truth chain: `mc2rs-gen` fetches the real `music.menu`
//! .ogg files through the asset index (sha1-verified) into `assets-cache/jar`
//! — this module decodes THOSE bytes and feeds the scheduler the REAL track
//! lengths. Output backends: `SilentOutput` (deterministic, default for
//! headless/CI and when no device exists) and `RodioOutput` (the real
//! speaker path, compiled behind the `audio` cargo feature because the OS
//! audio stack is a runtime environment property, not a code property).

use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context, Result};

/// A fully decoded, interleaved f32 track (-1.0..1.0).
#[derive(Debug, Clone)]
pub struct Track {
    pub sample_rate: u32,
    pub channels: usize,
    /// frames == samples.len() / channels (one frame = one sample per channel)
    pub frames: u64,
    pub samples: Vec<f32>,
}

/// Decode any format the symphonia default registry supports (ogg/vorbis for
/// the official music, wav/pcm for tests) into interleaved f32 samples.
pub fn decode_audio(bytes: &[u8]) -> Result<Track> {
    use symphonia::core::audio::SampleBuffer;
    use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
    use symphonia::core::errors::Error as SymphoniaError;
    use symphonia::core::formats::FormatOptions;
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    use symphonia::core::probe::Hint;

    anyhow::ensure!(!bytes.is_empty(), "no audio bytes");
    let mss = MediaSourceStream::new(
        Box::new(std::io::Cursor::new(bytes.to_vec())),
        Default::default(),
    );
    let probed = symphonia::default::get_probe()
        .format(
            &Hint::new(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .context("audio format probe")?;
    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .context("no audio track in stream")?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();

    let mut decoder = symphonia::default::get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .context("audio codec unsupported")?;

    let mut samples: Vec<f32> = Vec::new();
    let mut sample_rate = 0u32;
    let mut channels = 0usize;
    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            // symphonia signals clean end-of-stream as an UnexpectedEof io error
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(anyhow::Error::new(e).context("audio packet read")),
        };
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                sample_rate = spec.rate;
                channels = spec.channels.count();
                let mut buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
                buf.copy_interleaved_ref(decoded);
                samples.extend_from_slice(buf.samples());
            }
            // damaged single packets are skipped, like the vanilla sound engine
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(anyhow::Error::new(e).context("audio decode")),
        }
    }

    anyhow::ensure!(
        sample_rate > 0 && channels > 0 && !samples.is_empty(),
        "no audio frames decoded"
    );
    Ok(Track {
        sample_rate,
        channels,
        frames: (samples.len() / channels) as u64,
        samples,
    })
}

/// Output backends. `play` replaces whatever was playing (the vanilla menu
/// music is a sequence of full tracks; the scheduler decides the timing).
pub trait AudioOutput: Send {
    fn play(&mut self, track: &Track);
    fn stop(&mut self);
    fn playing(&self) -> bool;
}

/// Deterministic silent backend: records state, emits nothing. Default for
/// headless/CI and whenever no real device is available — the schedule and
/// event log stay byte-identical to the audible path.
#[derive(Default)]
pub struct SilentOutput {
    active: bool,
}

impl AudioOutput for SilentOutput {
    fn play(&mut self, _track: &Track) {
        self.active = true;
    }
    fn stop(&mut self) {
        self.active = false;
    }
    fn playing(&self) -> bool {
        self.active
    }
}

/// Real output through rodio/CPAL. Only compiled with `--features audio`;
/// at runtime `OutputStream::try_default()` fails gracefully on devices
/// without speakers and the caller falls back to `SilentOutput`.
#[cfg(feature = "audio")]
pub struct RodioOutput {
    // the stream must outlive the sink (dropping it closes the device)
    _stream: rodio::OutputStream,
    sink: rodio::Sink,
}

#[cfg(feature = "audio")]
impl RodioOutput {
    pub fn try_new() -> Result<Self> {
        let (_stream, handle) = rodio::OutputStream::try_default()
            .context("no audio output device")?;
        let sink = rodio::Sink::try_new(&handle).context("audio sink")?;
        Ok(Self { _stream, sink })
    }
}

#[cfg(feature = "audio")]
impl AudioOutput for RodioOutput {
    fn play(&mut self, track: &Track) {
        use rodio::Source;
        self.sink.clear();
        let src = rodio::buffer::SamplesBuffer::new(
            track.channels as u16,
            track.sample_rate,
            track.samples.clone(),
        );
        self.sink.append(src);
    }
    fn stop(&mut self) {
        self.sink.stop();
    }
    fn playing(&self) -> bool {
        !self.sink.empty()
    }
}

/// Best available backend: rodio when compiled in AND a device opens,
/// deterministic silence otherwise.
pub fn default_output() -> Box<dyn AudioOutput> {
    #[cfg(feature = "audio")]
    if let Ok(o) = RodioOutput::try_new() {
        return Box::new(o);
    }
    Box::new(SilentOutput::default())
}

/// Load + decode the official menu music tracks from the runtime asset root
/// (populated by `mc2rs-gen generate` through the asset index). Track ids are
/// resolved through the GENERATED sounds.json table, file bytes sha1-verified
/// at fetch time. Missing/undecodable tracks are skipped: audio must never
/// break the client.
pub fn load_menu_music(root: &Path, tracks: &[&'static str]) -> Vec<(&'static str, Track)> {
    use mc_registry::generated::sounds as snd;
    let mut out = Vec::new();
    for id in tracks {
        let Some(ev) = snd::event(id) else { continue };
        let Some(file) = snd::files(ev).first() else { continue };
        // sounds.json names are relative to assets/minecraft/sounds/, the
        // vanilla engine appends ".ogg" when the name has no extension
        let rel = if file.name.ends_with(".ogg") {
            file.name.to_string()
        } else {
            format!("{}.ogg", file.name)
        };
        let path = root.join("assets/minecraft/sounds").join(&rel);
        let Ok(bytes) = std::fs::read(&path) else { continue };
        match decode_audio(&bytes) {
            Ok(t) => out.push((*id, t)),
            Err(e) => eprintln!("      music: {} undecodable ({e})", path.display()),
        }
    }
    out
}

/// Menu music player: owns the output backend + decoded tracks and mirrors
/// the deterministic `MusicScheduler` event log onto real audio. Lives for
/// the whole GUI session (dropping it closes the device stream).
pub struct MenuMusic {
    output: Box<dyn AudioOutput>,
    tracks: HashMap<&'static str, Track>,
    seen: usize,
}

impl MenuMusic {
    /// Load real menu tracks (if the runtime assets exist), give the
    /// scheduler the REAL track lengths, and take the best output backend.
    /// Never fails: missing assets or no device degrade to silence.
    pub fn start(app: &mut crate::screens::App) -> Self {
        let tracks = app.music.menu_tracks().to_vec();
        let mut decoded = load_menu_music(Path::new("assets-cache/jar"), &tracks);
        for (id, t) in &decoded {
            app.music.set_track_frames(id, t.frames);
        }
        let tracks = decoded.drain(..).collect();
        Self { output: default_output(), tracks, seen: 0 }
    }

    /// Mirror new scheduler "play ..." events onto the output backend.
    /// Called once per frame; cheap when the event log did not grow.
    pub fn sync(&mut self, music: &crate::music::MusicScheduler) {
        let seen = self.seen.min(music.events.len());
        for ev in &music.events[seen..] {
            if let Some(id) = ev.strip_prefix("play ").and_then(|s| s.split(" @").next()) {
                if let Some(t) = self.tracks.get(id) {
                    self.output.play(t);
                }
            }
        }
        self.seen = music.events.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Synthesize a tiny mono 16-bit PCM WAV (pure bytes, no fixtures, no
    /// Mojang assets) and decode it back through the same probe path the
    /// official .ogg files take. 100 samples at 22050 Hz.
    fn synth_wav() -> Vec<u8> {
        const RATE: u32 = 22050;
        const N: usize = 100;
        let samples: Vec<i16> = (0..N).map(|i| ((i * 137) % 1000) as i16).collect();
        let data_len = (samples.len() * 2) as u32;
        let mut b = Vec::with_capacity(44 + samples.len() * 2);
        b.extend_from_slice(b"RIFF");
        b.extend_from_slice(&(36 + data_len).to_le_bytes());
        b.extend_from_slice(b"WAVE");
        b.extend_from_slice(b"fmt ");
        b.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
        b.extend_from_slice(&1u16.to_le_bytes()); // PCM
        b.extend_from_slice(&1u16.to_le_bytes()); // mono
        b.extend_from_slice(&RATE.to_le_bytes());
        b.extend_from_slice(&(RATE * 2).to_le_bytes()); // byte rate
        b.extend_from_slice(&2u16.to_le_bytes()); // block align
        b.extend_from_slice(&16u16.to_le_bytes()); // bits
        b.extend_from_slice(b"data");
        b.extend_from_slice(&data_len.to_le_bytes());
        for s in &samples {
            b.extend_from_slice(&s.to_le_bytes());
        }
        b
    }

    #[test]
    fn synth_wav_roundtrips_through_probe() {
        let t = decode_audio(&synth_wav()).expect("decode wav");
        assert_eq!(t.sample_rate, 22050);
        assert_eq!(t.channels, 1);
        assert_eq!(t.frames, 100);
        for (i, s) in t.samples.iter().enumerate() {
            let expect = (((i * 137) % 1000) as i16) as f32 / 32768.0;
            assert!((s - expect).abs() < 1.0 / 4096.0, "sample {i}: {s} vs {expect}");
        }
    }

    #[test]
    fn garbage_is_an_error_not_a_panic() {
        assert!(decode_audio(b"definitely not audio").is_err());
        assert!(decode_audio(&[]).is_err());
        // ogg container magic but no valid codec bisection inside
        assert!(decode_audio(b"OggS\x00\x02truncated").is_err());
    }

    #[test]
    fn silent_output_mirrors_state() {
        let t = decode_audio(&synth_wav()).unwrap();
        let mut o = SilentOutput::default();
        assert!(!o.playing());
        o.play(&t);
        assert!(o.playing());
        o.stop();
        assert!(!o.playing());
    }

    /// Real-data check (NOT run on CI): decode the actual official menu
    /// music fetched by `mc2rs-gen generate`. Run here with:
    ///   cargo test -p mc-client -- --ignored
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_official_menu_ogg_decodes() {
        // music.menu.menu1 is the canonical vanilla menu track
        let path = Path::new("assets-cache/jar/assets/minecraft/sounds/music/menu/menu1.ogg");
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("skipping: {path:?} not present (run mc2rs-gen generate)");
                return;
            }
        };
        let t = decode_audio(&bytes).expect("official menu1.ogg must decode");
        // menu tracks are minutes long, 44.1/48 kHz stereo
        assert!(t.frames > 60 * 30_000, "suspiciously short: {} frames", t.frames);
        assert!(t.sample_rate >= 32_000, "rate {}", t.sample_rate);
        assert!((1..=2).contains(&t.channels), "channels {}", t.channels);
        println!(
            "official menu1.ogg: {} frames ({:.1}s), {} Hz, {} ch",
            t.frames,
            t.frames as f64 / t.sample_rate as f64,
            t.sample_rate,
            t.channels
        );
    }
}
