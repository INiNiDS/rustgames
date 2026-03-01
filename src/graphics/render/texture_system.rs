use crate::error::GraphicsError;
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

    /// Loads a texture from raw bytes and registers it under `label`.
    ///
    /// # Errors
    /// Returns [`GraphicsError::TextureLoadFailed`] if the bytes cannot be decoded.
    pub fn load_texture(&mut self, bytes: &[u8], label: &str) -> Result<usize, GraphicsError> {
        let texture = Texture::from_bytes(&self.device, &self.queue, bytes, Some(label))
            .map_err(|e| GraphicsError::TextureLoadFailed(label.to_string(), e))?;

        self.textures.insert(label.to_string(), texture);
        Ok(self.textures.len() - 1)
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

    /// Loads all image files from `dir` (non-recursive).
    ///
    /// Errors are printed as diagnostics; loading continues for the remaining files.
    pub fn load_texture_dir(&mut self, dir: &str) {
        let files = match fs::read_dir(dir) {
            Ok(rd) => rd
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .map(|e| e.path())
                .collect::<Vec<_>>(),
            Err(e) => {
                eprintln!("{}", GraphicsError::FileReadFailed(PathBuf::from(dir), e));
                return;
            }
        };
        for file in files {
            match fs::read(&file) {
                Ok(bytes) => {
                    let label = file.to_string_lossy().into_owned();
                    if let Err(e) = self.load_texture(&bytes, &label) {
                        eprintln!("{e}");
                    }
                }
                Err(e) => eprintln!("{}", GraphicsError::FileReadFailed(file, e)),
            }
        }
    }

    /// Loads all image files from `dir` and all subdirectories (recursive).
    ///
    /// Errors are printed as diagnostics; loading continues for the remaining files.
    pub fn load_texture_dir_recursive(&mut self, dir: &str) {
        let files = walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(std::result::Result::ok)
            .filter(|entry| entry.path().is_file());
        for entry in files {
            let path = entry.path().to_owned();
            match fs::read(&path) {
                Ok(bytes) => {
                    let label = path.to_string_lossy().into_owned();
                    if let Err(e) = self.load_texture(&bytes, &label) {
                        eprintln!("{e}");
                    }
                }
                Err(e) => eprintln!("{}", GraphicsError::FileReadFailed(path, e)),
            }
        }
    }

    #[must_use]
    pub fn get_texture(&self, label: &str) -> Option<&Texture> {
        self.textures.get(label)
    }
}
