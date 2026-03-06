//! Graphics-related errors for [`crate::graphics::render::RenderSettings`] and [`crate::graphics::render::TextureSystem`].

use std::path::{Path, PathBuf};
use thiserror::Error;
use super::Diagnostic;

/// Errors [`crate::graphics::render::RenderSettings`] и [`crate::graphics::render::TextureSystem`].
#[derive(Debug, Error)]
pub enum GraphicsError {
    #[error("{}", Self::fmt_surface(_0))]
    SurfaceCreationFailed(#[source] wgpu::CreateSurfaceError),

    #[error("{}", Self::fmt_adapter())]
    AdapterNotFound,

    #[error("{}", Self::fmt_device(_0))]
    DeviceCreationFailed(#[source] wgpu::RequestDeviceError),

    #[error("{}", Self::fmt_surface_config())]
    SurfaceConfigMismatch,

    #[error("{}", Self::fmt_surface_acquire(_0))]
    SurfaceAcquireFailed(#[source] wgpu::SurfaceError),

    #[error("{}", Self::fmt_texture(_0, _1))]
    TextureLoadFailed(String, #[source] anyhow::Error),

    #[error("{}", Self::fmt_file(_0, _1))]
    FileReadFailed(PathBuf, #[source] std::io::Error),

    #[error("{}", Self::fmt_instance_overflow(*_0))]
    InstanceCountOverflow(usize),
}

impl GraphicsError {
    fn fmt_surface(source: &wgpu::CreateSurfaceError) -> String {
        Diagnostic {
            code: "G001",
            title: "Failed to create wgpu surface",
            location: "RenderSettings::init_graphics()",
            what: "wgpu could not create a rendering surface from the window handle",
            why:  "the window was destroyed before the surface was created, \
                   or the window system is not supported by wgpu on this platform",
            fix:  "ensure the winit window is fully initialised before calling Engine::new(); \
                   check that the required wgpu backend feature (Vulkan/Metal/DX12) is enabled",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_adapter() -> String {
        Diagnostic {
            code: "G002",
            title: "No suitable GPU adapter found",
            location: "RenderSettings::init_graphics()",
            what: "wgpu could not find a GPU that supports the required surface",
            why:  "outdated or missing GPU drivers, or running in a headless environment",
            fix:  "update your GPU drivers; on Linux install mesa (`apt install mesa-vulkan-drivers`); \
                   for CI/headless use `wgpu::Backends::GL` with a software renderer (llvmpipe)",
            note: None,
        }
        .to_string()
    }

    fn fmt_device(source: &wgpu::RequestDeviceError) -> String {
        Diagnostic {
            code: "G003",
            title: "Failed to create wgpu device",
            location: "RenderSettings::init_graphics()",
            what: "wgpu could not open the logical device on the selected adapter",
            why:  "the requested features or limits exceed what the hardware supports",
            fix:  "use `wgpu::DeviceDescriptor::default()` to request no extra features; \
                   consult `adapter.features()` to check available capabilities",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_surface_config() -> String {
        Diagnostic {
            code: "G004",
            title: "Surface/Adapter configuration mismatch",
            location: "RenderSettings::create_config()",
            what: "the surface has no compatible default configuration for the selected adapter",
            why:  "the surface format or present mode is not supported by this GPU+driver combination",
            fix:  "enumerate `surface.get_capabilities(adapter).formats` and pick a supported format manually",
            note: None,
        }
        .to_string()
    }

    fn fmt_surface_acquire(source: &wgpu::SurfaceError) -> String {
        let is_outdated = matches!(source, wgpu::SurfaceError::Outdated);
        Diagnostic {
            code: "G005",
            title: "Failed to acquire swap-chain frame",
            location: "Renderer::draw()",
            what: "wgpu could not obtain the next presentable texture from the surface",
            why:  if is_outdated {
                "the surface is outdated — this usually happens right after a window resize"
            } else {
                "the surface was lost or the GPU device was disconnected"
            },
            fix:  if is_outdated {
                "this frame will be skipped automatically; the surface will be reconfigured on the next resize event"
            } else {
                "handle `SurfaceError::Lost` by calling `surface.configure(&device, &config)` again; \
                 on `SurfaceError::OutOfMemory` consider reducing texture sizes"
            },
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_texture(label: &str, source: &anyhow::Error) -> String {
        Diagnostic {
            code: "G006",
            title: "Failed to load texture",
            location: "TextureSystem::load_texture()",
            what: &format!("could not create a GPU texture for `{label}`"),
            why:  "the byte slice is not a valid PNG/JPEG/BMP/WebP image, or it is empty",
            fix:  "verify the bytes come from a supported image format; \
                   use `image::load_from_memory` to pre-validate before passing to the engine",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_file(path: &Path, source: &std::io::Error) -> String {
        let p = path.display();
        Diagnostic {
            code: "G007",
            title: "Failed to read texture file",
            location: "TextureSystem::load_texture_dir()",
            what: &format!("could not read file `{p}`"),
            why:  "the file does not exist or the process lacks read permission",
            fix:  "check the path and ensure the asset files are shipped with the binary",
            note: Some(format!("{source}")),
        }
        .to_string()
    }

    fn fmt_instance_overflow(count: usize) -> String {
        Diagnostic {
            code: "G008",
            title: "Sprite instance count exceeds u32 limit",
            location: "Renderer::draw_scene_layers() / SpriteRenderer::create_quad_buffers()",
            what: &format!("cannot cast instance count `{count}` to u32"),
            why:  "the number of sprite instances in a single batch exceeds u32::MAX (4 294 967 295)",
            fix:  "split the batch into smaller chunks before passing to the renderer",
            note: None,
        }
        .to_string()
    }
}
