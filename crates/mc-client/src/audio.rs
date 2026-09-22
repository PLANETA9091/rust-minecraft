//! Real audio for the generated client: official menu + in-game music and
//! UI one-shots decoded with symphonia and played through pluggable output
//! backends.
//!
//! Generator-first truth chain: `mc2rs-gen` fetches the real `music.menu`,
//! `music.game` and `ui.button.click` .ogg files through the asset index
//! (sha1-verified) into `assets-cache/jar` — this module decodes THOSE bytes
//! and feeds the scheduler the REAL per-file lengths. Output backends:
//! `SilentOutput` (deterministic, default for headless/CI and when no device
//! exists) and `RodioOutput` (the real speaker path, compiled behind the
//! `audio` cargo feature because the OS audio stack is a runtime environment
//! property, not a code property).
//!
//! Memory policy: files are decoded on demand at play time (one track at a
//! time); startup only counts frames (`probe_frames`) so the scheduler gets
//! the real lengths without retaining hundreds of MB of PCM.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

fn open_decoder(
    bytes: &[u8],
) -> Result<(
    Box<dyn symphonia::core::codecs::Decoder>,
    Box<dyn symphonia::core::formats::FormatReader>,
    u32,
)> {
    use symphonia::core::codecs::CODEC_TYPE_NULL;
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
    let format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .context("no audio track in stream")?;
    let track_id = track.id;
    let codec_params = track.codec_params.clone();
    let decoder = symphonia::default::get_codecs()
        .make(&codec_params, &Default::default())
        .context("audio codec unsupported")?;
    Ok((decoder, format, track_id))
}

/// Decode any format the symphonia default registry supports (ogg/vorbis for
/// the official music, wav/pcm for tests) into interleaved f32 samples.
pub fn decode_audio(bytes: &[u8]) -> Result<Track> {
    use symphonia::core::audio::SampleBuffer;
    use symphonia::core::errors::Error as SymphoniaError;

    let (mut decoder, mut format, track_id) = open_decoder(bytes)?;
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

/// (sample_rate, frames) WITHOUT retaining samples — memory-light startup
/// probe so the scheduler gets the real lengths for the whole playlist.
pub fn probe_frames(bytes: &[u8]) -> Result<(u32, u64)> {
    use symphonia::core::audio::SampleBuffer;
    use symphonia::core::errors::Error as SymphoniaError;

    let (mut decoder, mut format, track_id) = open_decoder(bytes)?;
    let mut frames = 0u64;
    let mut rate = 0u32;
    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(SymphoniaError::IoError(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(anyhow::Error::new(e).context("audio packet read")),
        };
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(decoded) => {
                let spec = *decoded.spec();
                rate = spec.rate;
                let ch = spec.channels.count();
                let mut buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
                buf.copy_interleaved_ref(decoded);
                frames += (buf.samples().len() / ch.max(1)) as u64;
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(anyhow::Error::new(e).context("audio decode")),
        }
    }
    anyhow::ensure!(rate > 0 && frames > 0, "no audio frames decoded");
    Ok((rate, frames))
}

/// Output backends. `play` takes ownership of the decoded track and replaces
/// whatever was playing (the vanilla menu music is a sequence of full files;
/// the scheduler decides the timing); `play_ui` overlays a one-shot UI sound
/// WITHOUT interrupting the music — exactly how the vanilla sound engine
/// keeps music and UI clicks on independent channels. NOT Send: the CPAL
/// device stream must stay on the thread that opened it — the GUI loop is
/// single-threaded.
pub trait AudioOutput {
    fn play(&mut self, track: Track);
    fn stop(&mut self);
    #[cfg_attr(not(test), expect(dead_code))]
    fn playing(&self) -> bool;
    /// Fire-and-forget UI one-shot (ui.button.click), overlapping music.
    fn play_ui(&mut self, track: Track);
    #[cfg_attr(not(test), expect(dead_code))]
    fn ui_playing(&self) -> bool;
}

/// Deterministic silent backend: records state, emits nothing. Default for
/// headless/CI and whenever no real device is available — the schedule and
/// event log stay byte-identical to the audible path.
#[derive(Default)]
pub struct SilentOutput {
    active: bool,
    ui_active: bool,
}

impl AudioOutput for SilentOutput {
    fn play(&mut self, _track: Track) {
        self.active = true;
    }
    fn stop(&mut self) {
        self.active = false;
    }
    fn playing(&self) -> bool {
        self.active
    }
    fn play_ui(&mut self, _track: Track) {
        self.ui_active = true;
    }
    fn ui_playing(&self) -> bool {
        self.ui_active
    }
}

/// Real output through rodio/CPAL. Only compiled with `--features audio`;
/// at runtime `OutputStream::try_default()` fails gracefully on machines
/// without speakers and the caller falls back to `SilentOutput`.
#[cfg(feature = "audio")]
pub struct RodioOutput {
    // the stream must outlive the sinks (dropping it closes the device)
    _stream: rodio::OutputStream,
    sink: rodio::Sink,
    /// independent channel for UI one-shots so a click never cuts music
    ui_sink: rodio::Sink,
}

#[cfg(feature = "audio")]
impl RodioOutput {
    pub fn try_new() -> Result<Self> {
        let (_stream, handle) =
            rodio::OutputStream::try_default().context("no audio output device")?;
        let sink = rodio::Sink::try_new(&handle).context("audio sink")?;
        let ui_sink = rodio::Sink::try_new(&handle).context("ui audio sink")?;
        Ok(Self { _stream, sink, ui_sink })
    }
}

#[cfg(feature = "audio")]
impl AudioOutput for RodioOutput {
    fn play(&mut self, track: Track) {
        self.sink.clear();
        let src =
            rodio::buffer::SamplesBuffer::new(track.channels as u16, track.sample_rate, track.samples);
        self.sink.append(src);
    }
    fn stop(&mut self) {
        self.sink.stop();
    }
    fn playing(&self) -> bool {
        !self.sink.empty()
    }
    fn play_ui(&mut self, track: Track) {
        // append (never clear): one-shots must not interrupt music; the sink
        // drains itself when the short click finishes
        let src =
            rodio::buffer::SamplesBuffer::new(track.channels as u16, track.sample_rate, track.samples);
        self.ui_sink.append(src);
    }
    fn ui_playing(&self) -> bool {
        !self.ui_sink.empty()
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

/// Resolve the runtime paths of the official sound files for a list of
/// events through the GENERATED sounds.json table (every file of every
/// event, keys from `music::file_key()`). Files were fetched +
/// sha1-verified by `mc2rs-gen` through the asset index; missing files are
/// skipped silently — audio must never break the client.
pub fn discover_events(root: &Path, event_ids: &[&str]) -> Vec<(String, PathBuf)> {
    let mut out = Vec::new();
    for id in event_ids {
        out.extend(discover_event_files(root, id));
    }
    out
}

/// Resolve the runtime paths of ONE sound event's files through the
/// GENERATED sounds.json table (keys from `music::file_key()`). sounds.json
/// names are relative to assets/minecraft/sounds/, the vanilla engine
/// appends ".ogg" when the name has no extension. Missing files are skipped
/// silently — audio must never break the client.
pub fn discover_event_files(root: &Path, event_id: &str) -> Vec<(String, PathBuf)> {
    use crate::sound_table as snd;
    let mut out = Vec::new();
    let Some(ev) = snd::event(event_id) else { return out };
    for (idx, file) in snd::files(ev).iter().enumerate() {
        let rel = if file.name.ends_with(".ogg") {
            file.name.to_string()
        } else {
            format!("{}.ogg", file.name)
        };
        let path = root.join("assets/minecraft/sounds").join(&rel);
        if path.is_file() {
            out.push((crate::music::file_key(event_id, idx), path));
        }
    }
    out
}

/// Resolve the runtime path of ONE file of ONE sound event through the
/// GENERATED sounds.json table (the block-sound lane's on-demand form —
/// the block events are hundreds, the engine's log names only what
/// plays). `file_idx` rides the engine log's `#<file>` field. Missing
/// files (or out-of-range indexes) resolve to `None` silently — audio
/// must never break the client.
pub fn block_sound_file(root: &Path, event_id: &str, file_idx: usize) -> Option<PathBuf> {
    use crate::sound_table as snd;
    let ev = snd::event(event_id)?;
    let files = snd::files(ev);
    let file = files.get(file_idx)?;
    let rel = if file.name.ends_with(".ogg") {
        file.name.to_string()
    } else {
        format!("{}.ogg", file.name)
    };
    let path = root.join("assets/minecraft/sounds").join(&rel);
    if path.is_file() {
        Some(path)
    } else {
        None
    }
}

/// The vanilla channel's own fold on the decoded samples: the volume is
/// the gain (the SoundEngine's channel attenuation — the samples scale,
/// the length stands), the pitch is the playback rate (the SoundEngine's
/// AL_PITCH — a linear resample, the duration divides by the pitch).
/// Deterministic, pure, and exactly the arithmetic the real mixer speaks.
pub fn apply_gain_and_pitch(track: &Track, volume: f32, pitch: f32) -> Track {
    if volume == 1.0 && pitch == 1.0 {
        return track.clone();
    }
    let gain = volume.max(0.0);
    let ch = track.channels.max(1);
    if pitch > 0.0 && (pitch - 1.0).abs() > 1e-6 {
        // linear-interpolation resample: the output frame i reads the
        // source at i * pitch (a pitch of 0.5 plays twice as long, 2.0
        // half — the vanilla rate change's own law)
        let src_frames = (track.samples.len() / ch) as f64;
        let out_frames = ((src_frames / pitch as f64).floor() as usize).max(1);
        let mut samples = Vec::with_capacity(out_frames * ch);
        for i in 0..out_frames {
            let pos = i as f64 * pitch as f64;
            let i0 = pos.floor() as usize;
            let i1 = (i0 + 1).min(src_frames as usize - 1);
            let frac = (pos - i0 as f64) as f32;
            for c in 0..ch {
                let a = track.samples[i0 * ch + c];
                let b = track.samples[i1 * ch + c];
                samples.push((a + (b - a) * frac) * gain);
            }
        }
        Track {
            sample_rate: track.sample_rate,
            channels: track.channels,
            frames: out_frames as u64,
            samples,
        }
    } else {
        let mut out = track.clone();
        for s in &mut out.samples {
            *s *= gain;
        }
        out
    }
}

/// The positioned voice's own stereo fold on the decoded samples (the
/// lateral pan law `sound::lateral_pan` computes — the log's ` s` field
/// the backend mirrors): the MONO track the two-speaker pair becomes, the
/// per-ear gains the linear family's own slice keeps — the right source
/// (+1.0) the left speaker silences, the center (0.0) both ears keep
/// (OpenAL's own stereo rendering of a mono source, the front/back
/// symmetry the no-HRTF pipeline speaks). The STEREO track the identity
/// the stereo-direct rule keeps (OpenAL never positions a multi-channel
/// source — the pan field cannot fold), the center voice the identity the
/// no-op needs. Pure, deterministic, the length never moves.
pub fn apply_pan(track: &Track, pan: f32) -> Track {
    if pan == 0.0 || track.channels != 1 {
        return track.clone();
    }
    let left = 1.0 - pan.max(0.0);
    let right = 1.0 - (-pan).max(0.0);
    let mut samples = Vec::with_capacity(track.samples.len() * 2);
    for s in &track.samples {
        samples.push(s * left);
        samples.push(s * right);
    }
    Track {
        sample_rate: track.sample_rate,
        channels: 2,
        frames: track.frames,
        samples,
    }
}

/// The log line's own parameter tail: `"v0.250000 p0.500000"` the plain
/// lanes wear, `"... s+0.500000"` the panned voice's stereo tail rides —
/// (volume, pitch, pan) the fold's triple answers, the defaults the
/// identity values keep (1.0 / 1.0 / 0.0), the garbage the defaults
/// never let break a voice.
fn parse_voice_params(params: &str) -> (f32, f32, f32) {
    let mut volume = 1.0;
    let mut pitch = 1.0;
    let mut pan = 0.0;
    for (i, tok) in params.split_whitespace().enumerate() {
        if i == 0 {
            volume = tok.parse().unwrap_or(1.0);
        } else if let Some(p) = tok.strip_prefix('p') {
            pitch = p.parse().unwrap_or(1.0);
        } else if let Some(s) = tok.strip_prefix('s') {
            pan = s.parse().unwrap_or(0.0);
        }
    }
    (volume, pitch, pan)
}

/// Menu audio player: owns the output backend + the discovered file paths,
/// mirrors the deterministic `MusicScheduler` event log onto real music and
/// the App UI-sound log onto real one-shots (ui.button.click). Lives for the
/// whole GUI session (dropping it closes the device stream).
pub struct MenuMusic {
    output: Box<dyn AudioOutput>,
    files: HashMap<String, PathBuf>,
    seen: usize,
    /// ui.button.click files, keyed file_key() ("ui.button.click#0")
    ui_files: HashMap<String, PathBuf>,
    ui_seen: usize,
    /// the block-sound lane's own consume cursor (the engine log grows
    /// append-only; the mirror reads only the new entries)
    block_seen: usize,
}

impl MenuMusic {
    /// Discover the real menu music files (if the runtime assets exist),
    /// give the scheduler the REAL per-file lengths, and take the best
    /// output backend. Never fails: missing assets or no device degrade to
    /// silence. Startup cost: one frame-counting decode pass over the
    /// playlist (samples are not retained).
    pub fn start(app: &mut crate::screens::App) -> Self {
        Self::start_with_root(app, Path::new("assets-cache/jar"))
    }

    /// `start` against an explicit runtime asset root (tests redirect this
    /// to a temp tree holding synthesized sound files).
    pub fn start_with_root(app: &mut crate::screens::App, root: &Path) -> Self {
        let menu = app.music.menu_tracks().to_vec();
        let game = app.music.game_tracks().to_vec();
        // every event the official biome data references: ANY seed's spawn
        // biome must resolve to real paths + decoded lengths, whatever the
        // situation turns out to be
        let biome = crate::music::MusicScheduler::all_biome_events();
        let files = discover_events(
            root,
            &[menu.as_slice(), game.as_slice(), biome.as_slice()].concat(),
        );
        for (key, path) in &files {
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok((_, frames)) = probe_frames(&bytes) {
                    app.music.set_track_frames(key, frames);
                }
            }
        }
        let ui_files = discover_event_files(root, "ui.button.click").into_iter().collect();
        Self {
            output: default_output(),
            files: files.into_iter().collect(),
            seen: 0,
            ui_files,
            ui_seen: 0,
            block_seen: 0,
        }
    }

    /// Decode one official sound file and hand it to the backend (music
    /// channel or UI one-shot channel). Failures degrade to a stderr note —
    /// audio must never break the client.
    fn play_file(&mut self, key: &str, path: &Path, ui: bool) {
        match std::fs::read(path) {
            Ok(bytes) => match decode_audio(&bytes) {
                Ok(t) => {
                    if ui {
                        self.output.play_ui(t);
                    } else {
                        self.output.play(t);
                    }
                }
                Err(e) => eprintln!("      audio: {key} undecodable ({e})"),
            },
            Err(e) => eprintln!("      audio: {key} unreadable ({e})"),
        }
    }

    /// Mirror new scheduler events onto the music channel: "play
    /// <event#file>" decodes the chosen file on demand (one track in
    /// memory), "stop" interrupts whatever plays — vanilla stops music on
    /// situation change (entering/leaving a world). Called once per frame;
    /// cheap when the event log did not grow.
    pub fn sync(&mut self, music: &crate::music::MusicScheduler) {
        let seen = self.seen.min(music.events.len());
        for ev in &music.events[seen..] {
            if ev.starts_with("stop") {
                self.output.stop();
                continue;
            }
            if let Some(key) = ev.strip_prefix("play ").and_then(|s| s.split(" @").next()) {
                let Some(path) = self.files.get(key).cloned() else { continue };
                self.play_file(key, &path, false);
            }
        }
        self.seen = music.events.len();
    }

    /// Mirror new App UI-sound events ("ui.button.click @<frame>") onto the
    /// UI one-shot channel. The vanilla click event has exactly one file, so
    /// file #0 is the correct pick; unknown/garbled entries are skipped.
    pub fn sync_ui(&mut self, events: &[String]) {
        let seen = self.ui_seen.min(events.len());
        for ev in &events[seen..] {
            let Some(id) = ev.split(" @").next() else { continue };
            let key = crate::music::file_key(id, 0);
            let Some(path) = self.ui_files.get(&key).cloned() else { continue };
            self.play_file(&key, &path, true);
        }
        self.ui_seen = events.len();
    }

    /// Mirror new block-sound events ("{event} @{tick} #{file} v{vol}
    /// p{pitch}" — the block-sound lane's own log, the positioned pan
    /// lane's ` s{:+.6}` tail the stereo voices ride) onto the one-shot
    /// channel, the vanilla channel's gain + rate fold applied to the
    /// decoded official .ogg and the lateral pan the mono file's
    /// two-speaker split answers. The path resolves on demand through the
    /// generated table (hundreds of events, the log names only what
    /// plays); unreadable files are skipped — audio never breaks the
    /// client.
    pub fn sync_block_sounds(&mut self, events: &[String]) {
        let seen = self.block_seen.min(events.len());
        for ev in &events[seen..] {
            let Some((event, rest)) = ev.split_once(" @") else { continue };
            let Some((file_txt, params)) = rest.split_once(" v") else { continue };
            let Some(file_idx) = file_txt
                .strip_prefix('#')
                .and_then(|s| s.parse::<usize>().ok())
            else {
                continue;
            };
            let (volume, pitch, pan) = parse_voice_params(params);
            let Some(path) = block_sound_file(Path::new("assets-cache/jar"), event, file_idx)
            else {
                continue;
            };
            match std::fs::read(&path).ok().as_deref().map(decode_audio) {
                Some(Ok(t)) => {
                    let t = apply_gain_and_pitch(&t, volume, pitch);
                    let t = apply_pan(&t, pan);
                    self.output.play_ui(t);
                }
                other => {
                    if let Some(Err(e)) = other {
                        eprintln!("      audio: {event}#{file_idx} undecodable ({e})");
                    }
                }
            }
        }
        self.block_seen = events.len();
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
        // the probe agrees on rate + frame count without keeping samples
        let (rate, frames) = probe_frames(&synth_wav()).expect("probe wav");
        assert_eq!(rate, 22050);
        assert_eq!(frames, 100);
    }

    #[test]
    fn garbage_is_an_error_not_a_panic() {
        assert!(decode_audio(b"definitely not audio").is_err());
        assert!(decode_audio(&[]).is_err());
        // ogg container magic but no valid codec bisection inside
        assert!(decode_audio(b"OggS\x00\x02truncated").is_err());
        assert!(probe_frames(b"definitely not audio").is_err());
    }

    #[test]
    fn silent_output_mirrors_state() {
        let t = decode_audio(&synth_wav()).unwrap();
        let mut o = SilentOutput::default();
        assert!(!o.playing());
        o.play(t.clone());
        assert!(o.playing());
        o.play(t);
        assert!(o.playing());
        o.stop();
        assert!(!o.playing());
    }

    /// THE PARAM TAIL'S OWN WALK: the plain 2-token shape the block lanes
    /// wear, the panned 3-token shape the stereo tail rides, the garbage
    /// the identity defaults keep — the parse never lets a malformed line
    /// break a voice.
    #[test]
    fn parse_voice_params_walks_the_shapes() {
        // the plain pair the 4-field log lines carry
        let (v, p, s) = parse_voice_params("0.250000 p0.500000");
        assert_eq!(v, 0.25);
        assert_eq!(p, 0.5);
        assert_eq!(s, 0.0);
        // the panned triple the positioned lane's tail wears (the signed
        // pan the `{:+.6}` format speaks)
        let (v, p, s) = parse_voice_params("0.750000 p1.000000 s+0.325000");
        assert_eq!(v, 0.75);
        assert_eq!(p, 1.0);
        assert_eq!(s, 0.325);
        let (_, _, s) = parse_voice_params("1.000000 p1.000000 s-0.500000");
        assert_eq!(s, -0.5);
        // the defaults the identity keeps — the empty tail and the
        // garbage tokens never break the voice
        let (v, p, s) = parse_voice_params("");
        assert_eq!((v, p, s), (1.0, 1.0, 0.0));
        let (v, p, s) = parse_voice_params("not-a-number pwhat swhatever");
        assert_eq!((v, p, s), (1.0, 1.0, 0.0));
    }

    /// THE STEREO FOLD'S OWN WALK: the mono + pan the two-speaker pair
    /// becomes (the per-ear gains the linear slice keeps), the center and
    /// the stereo source the identity keeps, the length the fold never
    /// moves.
    #[test]
    fn apply_pan_folds_the_mono_pair_and_keeps_the_identities() {
        let mono = Track {
            sample_rate: 22050,
            channels: 1,
            frames: 3,
            samples: vec![1.0, 0.5, -0.25],
        };
        // the right-half pan: the left ear the linear slice halves, the
        // right ear the full keeps
        let panned = apply_pan(&mono, 0.5);
        assert_eq!(panned.channels, 2);
        assert_eq!(panned.frames, 3);
        assert_eq!(panned.sample_rate, 22050);
        assert_eq!(panned.samples, vec![0.5, 1.0, 0.25, 0.5, -0.125, -0.25]);
        // the pure left (-1.0): the left ear full, the right ear silent
        let left = apply_pan(&mono, -1.0);
        assert_eq!(left.samples, vec![1.0, 0.0, 0.5, 0.0, -0.25, 0.0]);
        // the pure right (+1.0): the left ear silent, the right ear full
        let right = apply_pan(&mono, 1.0);
        assert_eq!(right.samples, vec![0.0, 1.0, 0.0, 0.5, 0.0, -0.25]);
        // the center the identity keeps: the mono track the mono stays
        let center = apply_pan(&mono, 0.0);
        assert_eq!(center.channels, 1);
        assert_eq!(center.samples, mono.samples);
        // the stereo source the stereo-direct rule keeps (OpenAL never
        // positions a multi-channel source — the pan cannot fold)
        let stereo = Track {
            sample_rate: 22050,
            channels: 2,
            frames: 2,
            samples: vec![1.0, 0.0, 0.0, 1.0],
        };
        let kept = apply_pan(&stereo, 0.5);
        assert_eq!(kept.channels, 2);
        assert_eq!(kept.samples, stereo.samples);
    }

    #[test]
    fn ui_channel_is_independent_of_music() {
        let t = decode_audio(&synth_wav()).unwrap();
        let mut o = SilentOutput::default();
        o.play(t.clone());
        assert!(o.playing() && !o.ui_playing(), "music alone");
        o.play_ui(t);
        assert!(o.playing() && o.ui_playing(), "click overlays music, never cuts it");
        o.stop();
        assert!(!o.playing() && o.ui_playing(), "stopping music leaves the one-shot alone");
    }

    #[test]
    fn discover_skips_missing_event_and_files() {
        let dir = std::env::temp_dir().join(format!("mc2rs-disc-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // event exists in the generated table but the file is absent
        assert!(discover_event_files(&dir, "ui.button.click").is_empty());
        // event not in the generated table at all
        assert!(discover_event_files(&dir, "no.such.event").is_empty());
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn sync_ui_plays_click_on_the_ui_channel() {
        // temp runtime root holding a sound file at the exact official path
        // the loaded table resolves ui.button.click to (the REAL table's
        // random/click_stereo, the synthetic test boot's own file — the
        // path the table names, whatever boot speaks); a synthesized WAV
        // decodes fine — the probe reads content, not the file extension
        let dir = std::env::temp_dir().join(format!("mc2rs-click-{}", std::process::id()));
        use crate::sound_table as snd;
        let ev = snd::event("ui.button.click").expect("the click event the table speaks");
        let rel = snd::files(ev)[0].name.clone();
        let click_path = dir
            .join("assets/minecraft/sounds")
            .join(format!("{rel}.ogg"));
        std::fs::create_dir_all(click_path.parent().unwrap()).unwrap();
        std::fs::write(&click_path, synth_wav()).unwrap();

        let mut app = crate::screens::App::new(1);
        let mut m = MenuMusic::start_with_root(&mut app, &dir);
        // the App log is append-only; mirror exactly that protocol here
        let mut log: Vec<String> = Vec::new();
        m.sync_ui(&log);
        assert!(!m.output.ui_playing());
        // garbled + unknown entries are skipped, never fatal
        log.push("nonsense".to_string());
        log.push("not.an.event @5".to_string());
        m.sync_ui(&log);
        assert!(!m.output.ui_playing());
        assert!(!m.output.playing(), "music untouched by UI events");
        // the vanilla click plays through the independent UI channel
        log.push("ui.button.click @7".to_string());
        m.sync_ui(&log);
        assert!(m.output.ui_playing(), "click one-shot reaches the UI channel");
        assert!(!m.output.playing());
        // a second click replays (the log cursor advances)
        log.push("ui.button.click @8".to_string());
        m.sync_ui(&log);
        assert!(m.output.ui_playing());
        std::fs::remove_dir_all(&dir).ok();
    }

    /// Real-data check (NOT run on CI): discover + probe + decode the actual
    /// official menu music fetched by `mc2rs-gen generate`. Run here with:
    ///   cargo test -p mc-client -- --ignored
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_official_menu_music_decodes() {
        // cargo test runs from the package dir; the GUI runs from the repo root
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        // the scheduler knows the official pool from the generated table
        let sched_tracks = vec!["music.menu"];
        let files = discover_events(&root, &sched_tracks);
        assert_eq!(
            files.len(),
            9,
            "26.2 music.menu pool must resolve to 9 real files: {files:?}"
        );
        for (key, path) in &files {
            let bytes = std::fs::read(path).expect("read");
            let (rate, frames) = probe_frames(&bytes)
                .unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
            // menu tracks are minutes long, 44.1/48 kHz
            assert!(frames > 60 * 30_000, "{key}: suspiciously short ({frames} frames)");
            assert!(rate >= 32_000, "{key}: rate {rate}");
            let t = decode_audio(&bytes)
                .unwrap_or_else(|e| panic!("{key}: decode failed: {e}"));
            assert_eq!(t.frames, frames, "{key}: probe and decode must agree");
            println!(
                "  {key} <- {} ({} frames, {:.1}s, {} Hz, {} ch)",
                path.display(),
                t.frames,
                t.frames as f64 / t.sample_rate as f64,
                t.sample_rate,
                t.channels
            );
        }
    }

    /// Real-data check (NOT run on CI): the official ui.button.click file
    /// fetched by `mc2rs-gen generate` decodes and is click-length short.
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_official_ui_click_decodes() {
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        let files = discover_event_files(&root, "ui.button.click");
        assert_eq!(
            files.len(),
            1,
            "ui.button.click must resolve to its one official file: {files:?}"
        );
        let (key, path) = &files[0];
        assert_eq!(key, "ui.button.click#0");
        let bytes = std::fs::read(path).expect("read");
        let (rate, frames) =
            probe_frames(&bytes).unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
        let t = decode_audio(&bytes).unwrap_or_else(|e| panic!("{key}: decode failed: {e}"));
        assert_eq!(t.frames, frames, "{key}: probe and decode must agree");
        let secs = frames as f64 / rate as f64;
        assert!(secs > 0.0 && secs < 1.5, "a UI click is short, got {secs:.3}s");
        println!("  {key} <- {} ({frames} frames, {secs:.3}s, {rate} Hz)", path.display());
    }

    /// Real-data check (NOT run on CI): the zombie's four voice events —
    /// the ambient groan, the hurt, the death, and the playStepSound
    /// footstep — resolve to the official file counts and every file
    /// decodes (the mob-voice fetch the generator speaks).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_zombie_voice_oggs_decode() {
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        const EVENTS: [(&str, usize); 4] = [
            ("entity.zombie.ambient", 3),
            ("entity.zombie.hurt", 2),
            ("entity.zombie.death", 1),
            ("entity.zombie.step", 5),
        ];
        for (id, want) in EVENTS {
            let files = discover_event_files(&root, id);
            assert_eq!(
                files.len(),
                want,
                "{id} must resolve to {want} real files: {files:?}"
            );
            for (key, path) in &files {
                let bytes = std::fs::read(path).expect("read");
                let (rate, frames) = probe_frames(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
                let t = decode_audio(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: decode failed: {e}"));
                assert_eq!(t.frames, frames, "{key}: probe and decode must agree");
                // a voice or a footstep is short (the longest zombie
                // groan stays under three seconds)
                let secs = frames as f64 / rate as f64;
                assert!(secs > 0.0 && secs < 3.0, "{key}: got {secs:.3}s");
                println!("  {key} <- {} ({frames} frames, {secs:.3}s)", path.display());
            }
        }
    }

    /// Real-data check (NOT run on CI): the wolf's voice events — the
    /// 26.2 classic variant's ambient bark, the hurt, the death, the
    /// shared playStepSound footstep, AND the three state voices the
    /// ambient four-arm selection speaks (the angry growl, the
    /// tamed-hurt whine, the idle pant) — resolve to the official file
    /// counts and every file decodes (the mob-voice fetch the generator
    /// speaks; the classic directory the step event's variant-shared
    /// pool keeps).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_wolf_voice_oggs_decode() {
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        const EVENTS: [(&str, usize); 7] = [
            ("entity.wolf.ambient", 3),
            ("entity.wolf.hurt", 3),
            ("entity.wolf.death", 1),
            ("entity.wolf.step", 5),
            ("entity.wolf.growl", 3),
            ("entity.wolf.whine", 1),
            ("entity.wolf.pant", 1),
        ];
        for (id, want) in EVENTS {
            let files = discover_event_files(&root, id);
            assert_eq!(
                files.len(),
                want,
                "{id} must resolve to {want} real files: {files:?}"
            );
            for (key, path) in &files {
                let bytes = std::fs::read(path).expect("read");
                let (rate, frames) = probe_frames(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
                let t = decode_audio(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: decode failed: {e}"));
                assert_eq!(t.frames, frames, "{key}: probe and decode must agree");
                // a voice or a footstep is short (the longest wolf
                // death stays under three seconds)
                let secs = frames as f64 / rate as f64;
                assert!(secs > 0.0 && secs < 3.0, "{key}: got {secs:.3}s");
                println!("  {key} <- {} ({frames} frames, {secs:.3}s)", path.display());
            }
        }
    }

    /// Real-data check (NOT run on CI): the SEVEN remaining drawn kinds'
    /// voice events — the sheep's flat trio, the cow's classic and
    /// moody quads, the pig's classic/big/baby rows (the eat and step
    /// rows the sets speak), the chicken's classic/picky/baby rows, the
    /// skeleton's and the spider's flat quads, the creeper's own pair
    /// + the primed hiss, the explosion's own boom —
    /// the exact file counts the 26.2 sounds.json banks and every file
    /// decodes (the mob-voice fetch the generator's MOB_VOICE_EVENTS
    /// walk speaks; the same shape the zombie and wolf walks wear).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_seven_kind_voice_oggs_decode() {
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        const EVENTS: [(&str, usize); 48] = [
            ("entity.sheep.ambient", 3),
            ("entity.sheep.hurt", 3),
            ("entity.sheep.death", 3),
            ("entity.cow.ambient", 4),
            ("entity.cow.hurt", 3),
            ("entity.cow.death", 3),
            ("entity.cow.step", 4),
            ("entity.cow_moody.ambient", 9),
            ("entity.cow_moody.hurt", 4),
            ("entity.cow_moody.death", 2),
            ("entity.cow_moody.step", 4),
            ("entity.pig.ambient", 3),
            ("entity.pig.hurt", 3),
            ("entity.pig.death", 1),
            ("entity.pig.step", 5),
            ("entity.pig.eat", 2),
            ("entity.pig_big.ambient", 9),
            ("entity.pig_big.hurt", 3),
            ("entity.pig_big.death", 1),
            ("entity.pig_big.eat", 2),
            ("entity.baby_pig.ambient", 6),
            ("entity.baby_pig.hurt", 3),
            ("entity.baby_pig.death", 1),
            ("entity.baby_pig.step", 5),
            ("entity.baby_pig.eat", 2),
            ("entity.chicken.ambient", 3),
            ("entity.chicken.hurt", 2),
            ("entity.chicken.death", 2),
            ("entity.chicken.step", 2),
            ("entity.chicken_picky.ambient", 8),
            ("entity.chicken_picky.hurt", 3),
            ("entity.chicken_picky.death", 1),
            ("entity.baby_chicken.ambient", 2),
            ("entity.baby_chicken.hurt", 4),
            ("entity.baby_chicken.death", 1),
            ("entity.baby_chicken.step", 1),
            ("entity.skeleton.ambient", 3),
            ("entity.skeleton.hurt", 4),
            ("entity.skeleton.death", 1),
            ("entity.skeleton.step", 4),
            ("entity.spider.ambient", 4),
            ("entity.spider.hurt", 4),
            ("entity.spider.death", 1),
            ("entity.spider.step", 4),
            // the creeper's own pair + the primed hiss (the hurt the
            // say1..4 set, the death the own file, the primed the
            // random/fuse clip the single row banks — the sounds.json
            // rows the 26.2 banks)
            ("entity.creeper.hurt", 4),
            ("entity.creeper.death", 1),
            ("entity.creeper.primed", 1),
            // the explosion's own boom (the handleExplosion walk the
            // generic event plays — the four random/explode clips the
            // row banks)
            ("entity.generic.explode", 4),
        ];
        for (id, want) in EVENTS {
            let files = discover_event_files(&root, id);
            assert_eq!(
                files.len(),
                want,
                "{id} must resolve to {want} real files: {files:?}"
            );
            for (key, path) in &files {
                let bytes = std::fs::read(path).expect("read");
                let (rate, frames) = probe_frames(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
                let t = decode_audio(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: decode failed: {e}"));
                assert_eq!(t.frames, frames, "{key}: probe and decode must agree");
                // a voice, an eat, or a footstep is short — the
                // three-second sanity bound the zombie and wolf walks
                // keep, widened to 3.5s the seven-kind walk speaks: the
                // longest sound the table banks is the moody cow's
                // drawn-out moo (ambient5, 3.084s — the real blob the
                // official row keeps, the bound the honesty names)
                let secs = frames as f64 / rate as f64;
                assert!(secs > 0.0 && secs < 3.5, "{key}: got {secs:.3}s");
                println!("  {key} <- {} ({frames} frames, {secs:.3}s)", path.display());
            }
        }
    }

    /// Real-data check (NOT run on CI): the INTERACTION lane's voice
    /// events — the combat quartet, the vitals (the hurt pipeline, the
    /// death broadcast, the fall pair), the eat/burp, the
    /// levelup/pickup pair, the milk/saddle pair, the boat's two paddle
    /// lanes, the tool trio, the age-lock's own pair — the exact file
    /// counts the 26.2 sounds.json banks and every file decodes (the
    /// fetch the generator's INTERACTION_EVENTS walk speaks; the same
    /// shape the mob-voice walks wear).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_interaction_voice_oggs_decode() {
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        const EVENTS: [(&str, usize); 23] = [
            ("entity.player.attack.crit", 3),
            ("entity.player.attack.strong", 6),
            ("entity.player.attack.sweep", 7),
            ("entity.player.attack.weak", 4),
            ("entity.player.hurt", 3),
            ("entity.player.death", 3),
            ("entity.player.big_fall", 1),
            ("entity.player.small_fall", 1),
            ("entity.generic.eat", 3),
            ("entity.generic.drink", 1),
            ("item.honey_bottle.drink", 4),
            ("entity.player.burp", 1),
            ("entity.player.levelup", 1),
            ("entity.item.pickup", 1),
            ("entity.cow.milk", 3),
            ("entity.pig.saddle", 1),
            ("entity.boat.paddle_land", 6),
            ("entity.boat.paddle_water", 8),
            ("item.shovel.flatten", 4),
            ("item.hoe.till", 4),
            ("item.crop.plant", 12),
            ("item.golden_dandelion.use", 1),
            ("item.golden_dandelion.unuse", 1),
        ];
        for (id, want) in EVENTS {
            let files = discover_event_files(&root, id);
            assert_eq!(
                files.len(),
                want,
                "{id} must resolve to {want} real files: {files:?}"
            );
            for (key, path) in &files {
                let bytes = std::fs::read(path).expect("read");
                let (rate, frames) = probe_frames(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
                let t = decode_audio(&bytes)
                    .unwrap_or_else(|e| panic!("{key}: decode failed: {e}"));
                assert_eq!(t.frames, frames, "{key}: probe and decode must agree");
                // a hit, a paddle beat, a till, or a pickup blip is
                // short (the longest interaction sound stays under
                // three seconds — the levelup's chime the
                // random/levelup file keeps the bound answers)
                let secs = frames as f64 / rate as f64;
                assert!(secs > 0.0 && secs < 3.0, "{key}: got {secs:.3}s");
                println!("  {key} <- {} ({frames} frames, {secs:.3}s)", path.display());
            }
        }
    }

    /// Real-data check (NOT run on CI): the CAVE MOOD's own pool — the
    /// `ambient.cave` event the LEGACY_CAVE_SETTINGS machine plays, the
    /// 23 plain-string rows (`ambient/cave/cave1..23`) the 26.2
    /// sounds.json banks, every file decodes (the fetch the generator's
    /// AMBIENT_CAVE_EVENTS walk speaks; the same shape the mob-voice and
    /// interaction walks wear).
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_cave_mood_oggs_decode() {
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        let files = discover_event_files(&root, crate::sound::MOOD_EVENT);
        assert_eq!(files.len(), 23, "the cave pool's 23 rows resolve");
        for (key, path) in &files {
            let bytes = std::fs::read(path).expect("read");
            let (rate, frames) = probe_frames(&bytes)
                .unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
            let t =
                decode_audio(&bytes).unwrap_or_else(|e| panic!("{key}: decode failed: {e}"));
            assert_eq!(t.frames, frames, "{key}: probe and decode must agree");
            // a cave ambience is the pool's own long breath — the bound
            // the official durations keep, widened to 14s the pool's
            // longest file speaks: cave19's drawn-out wind (13.335s —
            // the moody cow's 3.5s the mob walk speaks is a VOICE bound,
            // the cave pool the deeper answer wears)
            let secs = frames as f64 / rate as f64;
            assert!(secs > 0.0 && secs < 14.0, "{key}: got {secs:.3}s");
            println!("  {key} <- {} ({frames} frames, {secs:.3}s)", path.display());
        }
    }

    #[test]
    fn sync_plays_and_stops_game_music() {
        // temp runtime root holding a sound file at EVERY official path the
        // music.game table resolves to, so the weighted pick
        // always lands on an existing file
        let dir = std::env::temp_dir().join(format!("mc2rs-game-{}", std::process::id()));
        use crate::sound_table as snd;
        // THE MIGRATION'S OWN LAW (the assert computes the expected from
        // the ACTUAL loaded state): the cache-absent boot the empty table
        // keeps — the sync walk below speaks where the cache exists
        let Some(ev) = snd::event("music.game") else {
            assert!(snd::events().is_empty(), "the empty table the honest arm keeps");
            return;
        };
        for f in snd::files(ev) {
            let p = dir
                .join("assets/minecraft/sounds")
                .join(format!("{}.ogg", f.name));
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, synth_wav()).unwrap();
        }

        let mut app = crate::screens::App::new(1);
        let mut m = MenuMusic::start_with_root(&mut app, &dir);
        // enter the world and wait (bounded) for the first game track to
        // start after its randomized delay — the OFFICIAL bounds are
        // 12000–24000 ticks in 26.2, so the bound must cover the max
        app.music.on_world_enter();
        let mut waited = 0u64;
        while !matches!(
            app.music.state(),
            crate::music::MusicState::Playing(..)
        ) {
            app.music.update();
            waited += 1;
            assert!(waited < 30_000, "game music never started");
        }
        m.sync(&app.music);
        assert!(m.output.playing(), "game track plays through the music channel");
        // returning to the title logs a stop: the game track is interrupted
        app.music.on_menu_open();
        m.sync(&app.music);
        assert!(!m.output.playing(), "stop event interrupts the game track");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn sync_plays_biome_situation_music() {
        // a biome whose DEFAULT event is small (26.2 swamp: 4 files in
        // music.overworld.swamp) — synthesized at every official path so the
        // weighted pick always lands, discovered through the SAME all-events
        // scan the real backend uses
        let dir = std::env::temp_dir().join(format!("mc2rs-biome-{}", std::process::id()));
        use crate::sound_table as snd;
        // THE MIGRATION'S OWN LAW (the assert computes the expected from
        // the ACTUAL loaded state): the cache-absent boot the empty table
        // keeps — the sync walk below speaks where the cache exists
        let Some(ev) = snd::event("music.overworld.swamp") else {
            assert!(snd::events().is_empty(), "the empty table the honest arm keeps");
            return;
        };
        assert!(ev.len > 0 && ev.len <= 8, "26.2 swamp pool: {} files", ev.len);
        for f in snd::files(ev) {
            let p = dir
                .join("assets/minecraft/sounds")
                .join(format!("{}.ogg", f.name));
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, synth_wav()).unwrap();
        }

        let mut app = crate::screens::App::new(1);
        let mut m = MenuMusic::start_with_root(&mut app, &dir);
        app.music.on_world_enter_biome("minecraft:swamp");
        let mut waited = 0u64;
        loop {
            match app.music.state() {
                crate::music::MusicState::Playing(t, _, _) => {
                    // Playing stores the EVENT id; the picked FILE appears
                    // in the event log ("play music.overworld.swamp#<i>")
                    assert_eq!(
                        *t, "music.overworld.swamp",
                        "the biome situation must play its own pool"
                    );
                    break;
                }
                _ => {
                    app.music.update();
                    waited += 1;
                    assert!(waited < 30_000, "biome music never started");
                }
            }
        }
        m.sync(&app.music);
        assert!(m.output.playing(), "biome track plays through the music channel");
        std::fs::remove_dir_all(&dir).ok();
    }

    /// Real-data check (NOT run on CI): the official music.game in-game
    /// pool (32 weighted files in 26.2) resolves and decodes.
    #[test]
    #[ignore = "needs runtime assets (assets-cache/jar) from mc2rs-gen generate"]
    fn real_official_game_music_resolves() {
        let candidates = [
            PathBuf::from("assets-cache/jar"),
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets-cache/jar"),
        ];
        let Some(root) = candidates.into_iter().find(|p| p.is_dir()) else {
            eprintln!("skipping: assets-cache/jar not present (run mc2rs-gen generate)");
            return;
        };
        let files = discover_event_files(&root, "music.game");
        assert_eq!(
            files.len(),
            32,
            "26.2 music.game pool must resolve to 32 real files: {files:?}"
        );
        for (key, path) in &files {
            let bytes = std::fs::read(path).expect("read");
            let (rate, frames) = probe_frames(&bytes)
                .unwrap_or_else(|e| panic!("{key}: probe failed: {e}"));
            assert!(frames > 60 * 30_000, "{key}: suspiciously short ({frames} frames)");
            assert!(rate >= 32_000, "{key}: rate {rate}");
        }
    }
}
