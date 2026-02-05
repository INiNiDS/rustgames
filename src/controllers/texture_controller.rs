use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use wgpu::{Device, Queue};
use std::sync::Arc;
use glam::Vec2;
use crate::graphics::Texture;
use crate::graphics::SpriteInstance;

pub struct TextureController {
    textures: HashMap<String, Texture>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    instances_per_texture: HashMap<String, Vec<SpriteInstance>>,
}

impl TextureController {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self {
            textures: HashMap::new(),
            device,
            queue,
            instances_per_texture: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, bytes: &[u8], label: &str) -> usize {
        let texture = Texture::from_bytes(&self.device, &self.queue, bytes, Option::from(label))
            .expect("Failed to load texture");

        self.textures.insert(label.to_string(), texture);
        self.textures.len() - 1
    }

    pub fn add_instance(&mut self, texture_label: &str, instance: SpriteInstance) {
        self.instances_per_texture
            .entry(texture_label.to_string())
            .or_insert_with(Vec::new)
            .push(instance);
    }

    pub fn use_texture(&mut self, label: &str, size: Vec2, position: Vec2, rotation: f32, opacity: f32) {
        let instance = SpriteInstance::simple(position, size, rotation, opacity);
        self.add_instance(label, instance);
    }

    pub fn get_batched_instances(&self) -> Vec<(&Texture, &[SpriteInstance])> {
        self.instances_per_texture
            .iter()
            .filter_map(|(label, instances)| {
                self.textures.get(label).map(|texture| (texture, instances.as_slice()))
            })
            .collect()
    }

    pub fn clear_instances(&mut self) {
        for instances in self.instances_per_texture.values_mut() {
            instances.clear();
        }
    }

    pub fn unload_texture(&mut self, label: &str) {
        self.textures.remove(label);
        self.instances_per_texture.remove(label);
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

    pub fn get_texture(&self, label: &str) -> Option<&Texture> {
        self.textures.get(label)
    }
}