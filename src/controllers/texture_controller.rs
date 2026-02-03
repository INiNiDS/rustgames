use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use wgpu::{Device, Queue};
use std::sync::Arc;
use glam::Vec2;
use crate::graphics::Texture;

#[derive(Clone, Copy)]
pub struct TextureDrawInfo {
    pub position: Vec2,
    pub size: Vec2,
}

pub struct TextureController {
    textures: HashMap<String, Texture>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    textures_to_use: HashMap<String, TextureDrawInfo>,
}

impl TextureController {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self {
            textures: HashMap::new(),
            device,
            queue,
            textures_to_use: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, bytes: &[u8], label: &str) -> usize {
        let texture = Texture::from_bytes(&self.device, &self.queue, bytes, Option::from(label))
            .expect("Failed to load texture");

        self.textures.insert(label.to_string(), texture);
        self.textures.len() - 1
    }

    pub fn use_texture(&mut self, label: &str, size: Vec2, position: Vec2) {
        self.textures_to_use.insert(
            label.to_string(),
            TextureDrawInfo {
                position,
                size,
            }
        );
    }

    pub fn get_textures_in_use(&self) -> Vec<(&Texture, Vec2, Vec2)> {
        self.textures_to_use.iter()
            .filter_map(|(label, info)| {
                self.textures.get(label).map(|texture| (texture, info.position, info.size))
            })
            .collect()
    }

    pub fn unload_texture(&mut self, label: &str) {
        self.textures.remove(label);
        self.textures_to_use.remove(label);
    }

    pub fn load_texture_dir(&mut self, dir: &str) {
        let files: Vec<PathBuf> = fs::read_dir(dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .map(|e| e.path())
            .collect();
        for file in files {
            let bytes = fs::read(&file).unwrap();
            self.load_texture(&bytes, file.to_str().unwrap());
        }
    }

    pub fn load_texture_dir_recursive(&mut self, dir: &str) {
        let files = walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file());
        for entry in files {
            let bytes = fs::read(&entry.path()).unwrap();
            self.load_texture(&bytes, entry.path().to_str().unwrap());
        }
    }
    
    /// Get a texture by label.
    pub fn get_texture(&self, label: &str) -> Option<&Texture> {
        self.textures.get(label)
    }
}