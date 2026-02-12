use crate::audio::audio_system::AudioSystem;
use crate::core::time::Time;
use crate::graphics::render::render_settings::RenderSettings;
use crate::graphics::render::renderer::Renderer;
use crate::graphics::{AnimationSystem, Camera, TextureSystem, VfxSystem};
use crate::text::text_system::TextSystem;
use crate::window::{Event, EventHandler, EventQueue, Window, WindowConfig};
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window as WinitWindow;

/// Central engine managing the window, renderer, input events, audio, and
/// per-frame timing. Provides accessor methods for every subsystem controller.
pub struct Engine {
    window: Window,
    render_settings: RenderSettings,
    time: Time,
    event_queue: EventQueue,
    handler_keys: Vec<Box<dyn EventHandler>>,
    audio_system: AudioSystem,
}

impl Engine {
    #[must_use]
    pub fn new(window: Arc<WinitWindow>) -> Self {
        let wrapped = Window::new(window.clone());

        let render_settings_future = RenderSettings::new(window);
        let render_settings = pollster::block_on(render_settings_future);
        Self {
            window: wrapped,
            render_settings,
            time: Time::new(),
            event_queue: EventQueue::new(),
            handler_keys: Vec::new(),
            audio_system: AudioSystem::new(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        Renderer::resize(&mut self.render_settings, new_size);
    }

    pub fn draw(&mut self) {
        Renderer::draw(&mut self.render_settings);
    }

    pub const fn delta_time(&self) -> f32 {
        self.time.delta_seconds()
    }

    pub const fn time(&self) -> &Time {
        &self.time
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn add_handler<T: EventHandler + 'static>(&mut self, key: T) {
        self.handler_keys.push(Box::new(key));
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    pub fn update(&mut self) {
        self.time.begin_frame();
        self.time.update();

        let events = self.event_queue.drain();
        for event in events {
            self.handle_event(event);
        }

        self.update_systems();
    }

    fn update_systems(&mut self) {
        let dt = self.delta_time();
        self.render_settings.get_text_system_mut().update(dt);
        self.render_settings.get_camera_mut().update(dt);
        self.render_settings.get_animation_system_mut().update(dt);
        self.render_settings.get_vfx_system_mut().update(dt);
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    pub fn set_window_config(&mut self, window_config: &WindowConfig) {
        self.render_settings.set_window_config(window_config);
    }

    fn handle_event(&mut self, event: Event) {
        for handler in &mut self.handler_keys {
            match event {
                Event::KeyPressed(key) => handler.on_key_pressed(key),
                Event::KeyReleased(key) => handler.on_key_released(key),
                Event::MouseMoved(x, y) => handler.on_mouse_moved(x, y),
                Event::MousePressed(button) => handler.on_mouse_pressed(button),
                Event::MouseReleased(button) => handler.on_mouse_released(button),
                Event::MouseWheel(delta) => handler.on_mouse_wheel(delta),
                Event::WindowClosed => handler.on_window_closed(),
                Event::WindowResized(w, h) => handler.on_window_resized(w, h),
                Event::WindowFocused(f) => handler.on_window_focused(f),
            }
        }
    }

    pub const fn get_text_system(&mut self) -> &mut TextSystem {
        self.render_settings.get_text_system_mut()
    }

    pub const fn get_camera(&mut self) -> &mut Camera {
        self.render_settings.get_camera_mut()
    }

    pub const fn get_event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

    pub const fn get_texture_controller(&mut self) -> &mut TextureSystem {
        self.render_settings.get_texture_controller_mut()
    }

    pub const fn get_audio_system(&mut self) -> &mut AudioSystem {
        &mut self.audio_system
    }

    pub const fn get_animation_system(&mut self) -> &mut AnimationSystem {
        self.render_settings.get_animation_system_mut()
    }

    pub const fn vfx_system(&mut self) -> &mut VfxSystem {
        self.render_settings.get_vfx_system_mut()
    }
}
