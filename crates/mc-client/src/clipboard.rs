//! THE OS CLIPBOARD'S OWN BRIDGE (the copypasta dep the headless risk
//! the feature gates — the audio law the bridge re-wears: the real
//! backend only `--features clipboard-os` compiles, the default build
//! the deterministic silence keeps, the in-memory store the walks' own
//! law the tests bank).

// the trait the real backend's own methods ride — the scope the
// feature-gated build the calls answer through
#[cfg(feature = "clipboard-os")]
use copypasta::ClipboardProvider;

/// THE OS STORE'S OWN READ (the feature the real backend answers — the
/// open failure or the empty daemon the None keeps, the default build
/// the None the deterministic silence wears).
pub fn os_get() -> Option<String> {
    #[cfg(feature = "clipboard-os")]
    {
        try_context().and_then(|mut c| c.get_contents().ok())
    }
    #[cfg(not(feature = "clipboard-os"))]
    {
        None
    }
}

/// THE OS STORE'S OWN WRITE (the feature the word pushes — the open or
/// set failure the false the caller ignores, the bank law the ride
/// never blocks).
pub fn os_set(word: &str) -> bool {
    #[cfg(feature = "clipboard-os")]
    {
        try_context()
            .map(|mut c| c.set_contents(word.to_string()).is_ok())
            .unwrap_or(false)
    }
    #[cfg(not(feature = "clipboard-os"))]
    {
        let _ = word;
        false
    }
}

/// THE COPY/CUT WALKS' OWN BANK (the store the whole word banks — the
/// law the walks wear — the OS the same word rides, the failure the
/// store answer never blocks).
pub fn store_bank(store: &mut String, word: &str) {
    *store = word.to_string();
    os_set(word);
}

/// THE PASTE WALKS' OWN LOAD (the non-empty store the word speaks —
/// the first-word law the walks wear — the empty store the OS pull
/// answers, the default build the empty string the deterministic
/// silence keeps).
pub fn store_load(store: &str) -> String {
    if !store.is_empty() {
        return store.to_string();
    }
    os_get().unwrap_or_default()
}

#[cfg(feature = "clipboard-os")]
fn try_context() -> Option<copypasta::ClipboardContext> {
    copypasta::ClipboardContext::new().ok()
}

#[cfg(all(test, not(feature = "clipboard-os")))]
mod tests {
    use super::*;

    #[test]
    fn the_default_bridge_the_deterministic_silence_keeps() {
        // the default build the OS never answers — the None the read
        // keeps, the false the write wears, the store law the helpers
        // bank unchanged
        assert_eq!(os_get(), None, "the default read the None keeps");
        assert!(!os_set("word"), "the default write the false wears");
        let mut store = String::from("old");
        store_bank(&mut store, "word");
        assert_eq!(store, "word", "the bank the same content stores");
        assert_eq!(
            store_load(&store),
            "word",
            "the non-empty store the load speaks"
        );
        assert_eq!(store_load(""), "", "the empty store the silence keeps");
    }
}
