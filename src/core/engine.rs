use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::core::time::Time;
use crate::graphics::renderer::Renderer;

pub struct Engine {
    window: Arc<Window>,
    renderer: Renderer,
    time: Time,
}

impl Engine {
    pub fn new(window: Arc<Window>) -> Self {
        let render_future = Renderer::new(window.clone());
        let renderer = pollster::block_on(render_future);
        Self { window, renderer, time: Time::new() }
    }


    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {

        self.renderer.resize(new_size);
    }

    pub fn draw(&mut self) {
        self.renderer.draw();
        let total_time = self.time.total_seconds();
        let x = (total_time.sin() + 1.0) / 2.0;
        self.window.set_title(&format!("Engine - Time: {:.2} s, x: {:.2}", total_time, x));
    }
    
    pub fn update(&mut self) {
        self.time.update();
    }
    
    pub fn delta_time(&self) -> f32 {
        self.time.delta_seconds()
    }
    
    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}