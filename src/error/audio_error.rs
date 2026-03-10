//! Audio-related errors with detailed diagnostics for troubleshooting and user guidance.

use super::Diagnostic;
use std::path::{Path, PathBuf};
use thiserror::Error;

type KiraSetupError = Box<dyn std::error::Error + Send + Sync + 'static>;
type KiraPlayError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Errors [`crate::audio::AudioSystem`].
#[derive(Debug, Error)]
pub enum AudioError {
    #[error("{}", Self::fmt_init(_0))]
    InitFailed(#[source] KiraSetupError),

    #[error("{}", Self::fmt_not_found(_0))]
    SoundNotFound(String),

    #[error("{}", Self::fmt_load(_0, _1))]
    LoadFailed(PathBuf, #[source] kira::sound::FromFileError),

    #[error("{}", Self::fmt_dir(_0, _1))]
    DirectoryReadFailed(PathBuf, #[source] std::io::Error),

    #[error("{}", Self::fmt_dir_entry(_0))]
    DirectoryEntryFailed(#[source] std::io::Error),

    #[error("{}", Self::fmt_playback(_0, _1))]
    PlaybackFailed(String, #[source] KiraPlayError),
}

impl AudioError {
    fn fmt_init(source: &KiraSetupError) -> String {
        Diagnostic {
            code: "A001",
            title: "Failed to initialize audio backend",
            location: "AudioSystem::new()",
            what: "the Kira audio manager could not start the DefaultBackend",
            why:  "no audio device is available, the driver crashed, or ALSA/PulseAudio is misconfigured",
            fix:  "check that your system has a working audio device; \
                   on Linux run `aplay -l` to list devices",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_not_found(name: &str) -> String {
        Diagnostic {
            code: "A002",
            title: "Sound asset not loaded",
            location: "AudioSystem::play()",
            what: &format!("sound `{name}` was not found in the loaded asset registry"),
            why:  &format!("you called `play(\"{name}\")` before `load_sound(\"{name}\", path)` or the name is misspelled"),
            fix:  &format!("add `audio.load_sound(\"{name}\", \"path/to/{name}.ogg\")` \
                            before calling `audio.play(\"{name}\")`"),
            note: None,
        }
        .to_string()
    }

    fn fmt_load(path: &Path, source: &kira::sound::FromFileError) -> String {
        let p = path.display();
        Diagnostic {
            code: "A003",
            title: "Failed to load sound file",
            location: "AudioSystem::load_sound()",
            what: &format!("could not decode the audio file `{p}`"),
            why: "the file does not exist, is not readable, or is not a valid WAV/OGG/MP3",
            fix: "verify the path is correct and the file is an uncompressed WAV, \
                   Vorbis OGG, or MPEG Layer-3 MP3",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_dir(path: &Path, source: &std::io::Error) -> String {
        let p = path.display();
        Diagnostic {
            code: "A004",
            title: "Failed to read sound directory",
            location: "AudioSystem::load_sound_dir()",
            what: &format!("could not open directory `{p}`"),
            why: "the directory does not exist or the process lacks read permission",
            fix: "check the path exists and the user has `r-x` permissions on the directory",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_dir_entry(source: &std::io::Error) -> String {
        Diagnostic {
            code: "A005",
            title: "Failed to read directory entry",
            location: "AudioSystem::load_sound_dir()",
            what: "a filesystem entry inside the sound directory could not be read",
            why: "the entry was deleted between listing and reading, or permissions changed",
            fix: "ensure no other process modifies the directory while loading assets",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_playback(name: &str, source: &KiraPlayError) -> String {
        Diagnostic {
            code: "A006",
            title: "Failed to play sound",
            location: "AudioSystem::play()",
            what: &format!("Kira could not start playback of `{name}`"),
            why: "the audio backend may have been shut down or reached the sound limit",
            fix: "call `stop_all()` to free handles before playing new sounds, \
                   or increase `AudioManagerSettings::num_sounds`",
            note: Some(format!("{source}")),
        }
        .to_string()
    }
}
