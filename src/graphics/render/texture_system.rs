use crate::graphics::SpriteInstance;
use crate::graphics::Texture;
use glam::Vec2;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use wgpu::{Device, Queue};

/// Manages GPU textures and per-frame sprite instance batching. Supports
/// loading from files, bytes, and directories.
pub struct TextureSystem {
    textures: HashMap<String, Texture>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    instances_per_texture: HashMap<String, Vec<SpriteInstance>>,
    frame_draw_order: Vec<String>,
}

impl TextureSystem {
    #[must_use]
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> Self {
        Self {
            textures: HashMap::new(),
            device,
            queue,
            instances_per_texture: HashMap::new(),
            frame_draw_order: Vec::new(),
        }
    }

    pub fn load_texture(&mut self, bytes: &[u8], label: &str) -> usize {
        let texture = Texture::from_bytes(&self.device, &self.queue, bytes, Some(label))
            .expect("Failed to load texture");

        self.textures.insert(label.to_string(), texture);
        self.textures.len() - 1
    }

    pub fn add_instance(&mut self, texture_label: &str, instance: SpriteInstance) {
        if !self.instances_per_texture.contains_key(texture_label) {
            self.frame_draw_order.push(texture_label.to_string());
            self.instances_per_texture
                .insert(texture_label.to_string(), Vec::new());
        }

        if let Some(batch) = self.instances_per_texture.get_mut(texture_label) {
            batch.push(instance);
        }
    }

    pub fn use_texture(
        &mut self,
        label: &str,
        size: Vec2,
        position: Vec2,
        rotation: f32,
        opacity: f32,
    ) {
        let instance = SpriteInstance::simple(position, size, rotation, opacity);
        self.add_instance(label, instance);
    }

    #[must_use]
    pub fn get_batched_instances(&self) -> Vec<(&Texture, &[SpriteInstance])> {
        self.frame_draw_order
            .iter()
            .filter_map(|label| {
                let texture = self.textures.get(label)?;
                let instances = self.instances_per_texture.get(label)?;
                Some((texture, instances.as_slice()))
            })
            .collect()
    }

    pub fn clear_instances(&mut self) {
        for instances in self.instances_per_texture.values_mut() {
            instances.clear();
        }
        self.instances_per_texture.clear();
        self.frame_draw_order.clear();
    }

    pub fn unload_texture(&mut self, label: &str) {
        self.textures.remove(label);
        self.instances_per_texture.remove(label);
        if let Some(pos) = self.frame_draw_order.iter().position(|x| x == label) {
            self.frame_draw_order.remove(pos);
        }
    }

    pub fn load_texture_dir(&mut self, dir: &str) {
        let files: Vec<PathBuf> = fs::read_dir(dir)
            .unwrap()
            .filter_map(std::result::Result::ok)
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
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().is_file());
        for entry in files {
            let bytes = fs::read(entry.path()).unwrap();
            self.load_texture(&bytes, entry.path().to_str().unwrap());
        }
    }

    #[must_use]
    pub fn get_texture(&self, label: &str) -> Option<&Texture> {
        self.textures.get(label)
    }
}
