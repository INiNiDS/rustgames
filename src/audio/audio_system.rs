use kira::backend::DefaultBackend;
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle};
use kira::{AudioManager, Tween};
use kira::AudioManagerSettings;
use std::collections::HashMap;
use std::sync::Arc;

pub struct AudioSystem {
    manager: AudioManager,
    sound_assets: HashMap<String, StaticSoundData>,
    active_sounds: HashMap<String, Arc<StaticSoundHandle>>,
}

impl AudioSystem {
    pub fn new() -> Self {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("Failed to initialize audio");
        Self {
            manager,
            sound_assets: HashMap::new(),
            active_sounds: HashMap::new(),
        }
    }

    pub fn load_sound(&mut self, name: &str, path: &str) {
        if self.sound_assets.contains_key(name) {
            return;
        }

        let data = StaticSoundData::from_file(path)
            .expect("Failed to load sound");
        self.sound_assets.insert(name.to_string(), data);
    }

    pub fn load_sound_dir(&mut self, dir_path: &str) {
        let paths = std::fs::read_dir(dir_path).expect("Failed to read sound directory");
        for entry in paths {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "wav" || extension == "ogg" || extension == "mp3" {
                        let file_stem = path.file_stem().unwrap().to_str().unwrap();
                        self.load_sound(file_stem, path.to_str().unwrap());
                    }
                }
            }
        }
    }

    pub fn load_sound_dir_recursive(&mut self, dir_path: &str) {
        let paths = walkdir::WalkDir::new(dir_path).into_iter();
        for entry in paths {
            self.load_sound_dir(&entry.unwrap().path().to_str().unwrap());
        }
    }

    pub fn play(&mut self, name: &str) -> Arc<StaticSoundHandle> {
        let data = self.sound_assets.get(name)
            .unwrap_or_else(|| panic!("Sound '{}' not loaded", name));

        if let Some(handle) = self.active_sounds.get(name) {
            if handle.state() == kira::sound::PlaybackState::Playing {
                return Arc::clone(handle);
            }
        }

        let handle = self.manager.play(data.clone()).expect("Failed to play sound");
        let shared_handle = Arc::new(handle);

        self.active_sounds.insert(name.to_string(), Arc::clone(&shared_handle));

        shared_handle
    }

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

            let mut handle_mut: StaticSoundHandle = Arc::try_unwrap(handle).expect("Failed to unwrap Arc");

            handle_mut.stop(tween);
        }
    }

    pub fn stop_all(&mut self) {
        for (_, handle) in self.active_sounds.drain() {
            let mut handle_mut = Arc::try_unwrap(handle).expect("Failed to unwrap Arc");
            handle_mut.stop(Tween::default());
        }
    }

    pub fn unload_sound(&mut self, name: &str) {
        self.sound_assets.remove(name);
    }
}