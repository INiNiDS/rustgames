use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::core::time::Time;
use crate::graphics::renderer::Renderer;
use crate::text::{Font, TextSpeed, TypewriterEffect};


pub struct Engine {
    window: Arc<Window>,
    renderer: Renderer,
    time: Time,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Self {
        let render_future = Renderer::new(window.clone());
        let renderer = pollster::block_on(render_future);
        Self { 
            window, 
            renderer, 
            time: Time::new() 
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    pub fn draw(&mut self) {
        self.renderer.draw();
    }
    
    pub fn update(&mut self) {
        self.time.begin_frame();
        self.time.update();
        self.renderer.camera.update(self.delta_time());
    }
    
    pub fn delta_time(&self) -> f32 {
        self.time.delta_seconds()
    }
    
    pub fn time(&self) -> &Time {
        &self.time
    }
    
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
    
    pub fn set_font(&mut self, font: Font) {
        self.renderer.set_font(font);
    }
    
    pub fn set_text_style(&mut self, style: crate::text::TextStyle) {
        self.renderer.set_text_style(style);
    }
    
    pub fn set_text(&mut self, text: &str, x: f32, y: f32, speed: TextSpeed) {
        self.renderer.set_text(&*text, x, y, speed);
    }
    
    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }
}
