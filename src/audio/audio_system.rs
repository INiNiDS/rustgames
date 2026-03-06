use crate::error::AudioError;
use kira::AudioManagerSettings;
use kira::backend::DefaultBackend;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::{AudioManager, Tween};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Manages audio playback using the Kira audio engine. Sounds are loaded as
/// named assets and can be played, stopped, or faded out.
pub struct AudioSystem {
    manager: AudioManager,
    sound_assets: HashMap<String, StaticSoundData>,
    active_sounds: HashMap<String, Arc<StaticSoundHandle>>,
}

impl AudioSystem {
    /// Creates a new [`AudioSystem`], initializing the audio backend.
    ///
    /// # Errors
    /// Returns [`AudioError::InitFailed`] when the Kira backend cannot start.
    pub fn new() -> Result<Self, AudioError> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .map_err(|e| AudioError::InitFailed(Box::new(e)))?;
        Ok(Self {
            manager,
            sound_assets: HashMap::new(),
            active_sounds: HashMap::new(),
        })
    }

    /// Loads a sound file and registers it under `name`.
    ///
    /// # Errors
    /// Returns [`AudioError::LoadFailed`] if the file cannot be decoded.
    pub fn load_sound(&mut self, name: &str, path: &str) -> Result<(), AudioError> {
        if self.sound_assets.contains_key(name) {
            return Ok(());
        }
        let data = StaticSoundData::from_file(path)
            .map_err(|e| AudioError::LoadFailed(PathBuf::from(path), e))?;
        self.sound_assets.insert(name.to_string(), data);
        Ok(())
    }

    /// Loads all supported audio files from a directory (non-recursive).
    ///
    /// # Errors
    /// Returns [`AudioError::DirectoryReadFailed`] if the directory cannot be opened,
    /// or propagates per-entry errors.
    pub fn load_sound_dir(&mut self, dir_path: &str) -> Result<(), AudioError> {
        let paths = std::fs::read_dir(dir_path)
            .map_err(|e| AudioError::DirectoryReadFailed(PathBuf::from(dir_path), e))?;
        for entry in paths {
            self.check_and_load_file(entry)?;
        }
        Ok(())
    }

    /// Loads all supported audio files from a directory tree (recursive).
    ///
    /// # Panics
    /// Propagates [`AudioError`] from [`Self::load_sound_dir`].
    /// # Errors
    /// Returns [`AudioError::DirectoryReadFailed`] if any directory cannot be opened,
    /// or propagates per-entry errors.
    pub fn load_sound_dir_recursive(&mut self, dir_path: &str) -> Result<(), AudioError> {
        for entry in walkdir::WalkDir::new(dir_path) {
            let entry = entry.map_err(|e| {
                AudioError::DirectoryReadFailed(
                    PathBuf::from(dir_path),
                    e.into_io_error().unwrap_or_else(|| {
                        std::io::Error::other("walkdir entry failed with non-io error")
                    }),
                )
            })?;
            if entry.file_type().is_dir() && entry.path().to_str().is_some() {
                let path = entry.path().to_str().unwrap();
                self.load_sound_dir(path)?;
            }
        }
        Ok(())
    }


    /// Plays a previously loaded sound by name.
    ///
    /// # Errors
    /// - [`AudioError::SoundNotFound`] — sound was never loaded.
    /// - [`AudioError::PlaybackFailed`] — Kira could not start playback.
    pub fn play(&mut self, name: &str) -> Result<Arc<StaticSoundHandle>, AudioError> {
        let data = self
            .sound_assets
            .get(name)
            .ok_or_else(|| AudioError::SoundNotFound(name.to_string()))?;

        if let Some(handle) = self.active_sounds.get(name)
            && handle.state() == kira::sound::PlaybackState::Playing
        {
            return Ok(Arc::clone(handle));
        }

        let handle = self
            .manager
            .play(data.clone())
            .map_err(|e| AudioError::PlaybackFailed(name.to_string(), Box::new(e)))?;
        let shared_handle = Arc::new(handle);
        self.active_sounds
            .insert(name.to_string(), Arc::clone(&shared_handle));
        Ok(shared_handle)
    }

    /// Stops a sound by name with an optional fade-out.
    pub fn stop(&mut self, name: &str, fade_out_secs: f64) {
        if let Some(handle) = self.active_sounds.remove(name) {
            let tween = if fade_out_secs > 0.0 {
                Tween {
                    duration: std::time::Duration::from_secs_f64(fade_out_secs),
                    ..Default::default()
                }
            } else {
                Tween::default()
            };
            if let Ok(mut h) = Arc::try_unwrap(handle) {
                h.stop(tween);
            }
        }
    }

    /// Stops all currently active sounds immediately.
    pub fn stop_all(&mut self) {
        for (_, handle) in self.active_sounds.drain() {
            if let Ok(mut h) = Arc::try_unwrap(handle) {
                h.stop(Tween::default());
            }
        }
    }

    /// Removes a loaded sound asset from memory.
    pub fn unload_sound(&mut self, name: &str) {
        self.sound_assets.remove(name);
    }

    fn check_and_load_file(
        &mut self,
        entry: Result<std::fs::DirEntry, std::io::Error>,
    ) -> Result<(), AudioError> {
        let entry = entry.map_err(AudioError::DirectoryEntryFailed)?;
        let path = entry.path();
        if path.is_file()
            && let Some(ext) = path.extension()
            && (ext == "wav" || ext == "ogg" || ext == "mp3")
        {
            // file_stem() is Some when the path has a file name — guaranteed above
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let path_str = path.to_string_lossy().into_owned();
            self.load_sound(&stem, &path_str)?;
        }
        Ok(())
    }
}
