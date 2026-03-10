use crate::audio::audio_system::AudioSystem;
use crate::core::time::Time;
use crate::error::GameError;
use crate::graphics::render::render_settings::RenderSettings;
use crate::graphics::render::renderer::Renderer;
use crate::graphics::{AnimationSystem, Camera, TextureSystem, VfxSystem};
use crate::text::text_system::TextSystem;
use crate::translation::{
    DictionarySystem, Language, LanguageSystem, Translation, TranslationSystem,
};
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
    /// Creates a new [`Engine`] bound to `window`.
    ///
    /// # Errors
    /// Returns [`GameError`] if GPU initialization or the audio backend fail.
    pub fn new(window: Arc<WinitWindow>) -> Result<Self, GameError> {
        let wrapped = Window::new(window.clone());

        let render_settings_future = RenderSettings::new(window);
        let render_settings = pollster::block_on(render_settings_future)?;
        Ok(Self {
            window: wrapped,
            render_settings,
            time: Time::new(),
            event_queue: EventQueue::new(),
            handler_keys: Vec::new(),
            audio_system: AudioSystem::new()?,
        })
    }

    /// Resizes the swap-chain and camera viewport to `new_size`.
    ///
    /// Called automatically by the windowing layer on `WindowEvent::Resized`.
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        Renderer::resize(&mut self.render_settings, new_size);
    }

    /// Encodes and submits the current frame to the GPU.
    pub fn draw(&mut self) {
        Renderer::draw(&mut self.render_settings);
    }

    /// Returns the elapsed time between the last two frames in seconds.
    pub const fn delta_time(&self) -> f32 {
        self.time.delta_seconds()
    }

    /// Returns a reference to the per-frame [`Time`] tracker.
    pub const fn time(&self) -> &Time {
        &self.time
    }

    /// Asks the operating system to schedule a redraw for the window.
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    /// Registers an [`EventHandler`] that will receive input callbacks each
    /// frame.
    pub fn add_handler<T: EventHandler + 'static>(&mut self, key: T) {
        self.handler_keys.push(Box::new(key));
    }

    /// Updates the title bar of the window.
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }

    /// Advances all subsystems for the current frame: timing, event
    /// dispatching, text, camera, animations, and VFX.
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

    /// Pushes a raw [`Event`] into the event queue to be dispatched this frame.
    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    /// Applies display settings (vsync, background colour, etc.) from
    /// `window_config` to the renderer.
    pub fn set_window_config(&mut self, window_config: &WindowConfig) {
        self.render_settings.set_window_config(window_config);
    }

    fn handle_event(&mut self, event: Event) {
        for handler in &mut self.handler_keys {
            Self::dispatch_event(handler.as_mut(), event);
        }
    }

    fn dispatch_event(handler: &mut dyn EventHandler, event: Event) {
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

    /// Returns a mutable reference to the [`TextSystem`] for queuing and
    /// rendering text.
    pub const fn get_text_system(&mut self) -> &mut TextSystem {
        self.render_settings.get_text_system_mut()
    }

    /// Returns a mutable reference to the [`Camera`] for 2D view control.
    pub const fn get_camera(&mut self) -> &mut Camera {
        self.render_settings.get_camera_mut()
    }

    /// Returns a shared reference to the [`EventQueue`] for polling input state.
    pub const fn get_event_queue(&self) -> &EventQueue {
        &self.event_queue
    }

    /// Returns a mutable reference to the [`TextureSystem`] for loading and
    /// drawing textures.
    pub const fn get_texture_controller(&mut self) -> &mut TextureSystem {
        self.render_settings.get_texture_controller_mut()
    }

    /// Returns a mutable reference to the [`AudioSystem`] for sound playback.
    pub const fn get_audio_system(&mut self) -> &mut AudioSystem {
        &mut self.audio_system
    }

    /// Returns a mutable reference to the [`AnimationSystem`] for managing
    /// sprite and property animations.
    pub const fn get_animation_system(&mut self) -> &mut AnimationSystem {
        self.render_settings.get_animation_system_mut()
    }

    /// Returns a mutable reference to the [`VfxSystem`] for particle and
    /// screen-space visual effects.
    pub const fn get_vfx_system(&mut self) -> &mut VfxSystem {
        self.render_settings.get_vfx_system_mut()
    }

    /// Stores a complete [`DictionarySystem`] and [`TranslationSystem`] in the
    /// renderer, merging them with any previously saved data.
    pub fn save_translations(
        &mut self,
        dictionary_system: DictionarySystem,
        translation: TranslationSystem,
    ) {
        self.render_settings.translation_system += translation;
        self.render_settings.dictionary_system += dictionary_system;
    }

    /// Adds a single [`Translation`] entry to the translation system.
    pub fn add_translation(&mut self, translation: Translation) {
        self.render_settings
            .translation_system
            .add_translation(translation);
    }

    /// Merges an entire [`DictionarySystem`] into the engine's dictionary.
    pub fn add_dictionary(&mut self, dictionary: DictionarySystem) {
        self.render_settings.dictionary_system += dictionary;
    }

    /// Register a language so it can be activated via `set_language`.
    pub fn add_language(&mut self, language: Language) {
        self.render_settings.language_system.add_language(language);
    }

    /// Activate a language by its `small_name` (e.g. `"en_us"`, `"ru_ru"`).
    pub fn set_language(&mut self, small_name: &str) {
        self.render_settings
            .language_system
            .set_current_language_by_name(small_name);
    }

    /// Access the language system directly.
    pub const fn get_language_system(&mut self) -> &mut LanguageSystem {
        &mut self.render_settings.language_system
    }
}
