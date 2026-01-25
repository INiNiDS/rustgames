use std::error::Error;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::core::engine::Engine;
use crate::text::{Font, TextSpeed};

pub struct App {
    engine: Option<Engine>,
    window_attributes: Option<WindowAttributes>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = self.window_attributes.take().unwrap_or_default();
        let window_result = event_loop.create_window(attributes);
        match window_result {
            Ok(window) => {
                let window = Arc::new(window);
                let engine = Engine::new(window);
                self.engine = Some(engine);
            }
            Err(e) => {
                eprintln!("Failed to create window: {:?}", e);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent
    ) {
        let engine = match self.engine.as_mut() {
            Some(engine) => engine,
            None => return,
        };
        
        match event {
            WindowEvent::Resized(new_size) => {
                engine.resize(new_size);
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                engine.update();
                engine.draw();
                engine.request_redraw();
            },
            _ => {},
        }
    }
}

impl App {
    pub fn set_font(&mut self, font_path: &str) {
        if let Some(engine) = self.engine.as_mut() {
            engine.set_font(Font::load(font_path).unwrap())
        }
    }
    pub fn set_text(&mut self, text: &str, x: f32, y: f32, speed: TextSpeed) {
        if let Some(engine) = self.engine.as_mut() {
            engine.set_text(&*text.to_string(), x, y, speed)
        }
    }
    pub fn set_text_style(&mut self, font_path: &str) {
        if let Some(engine) = self.engine.as_mut() {
            engine.set_font(Font::load(font_path).unwrap())
        }
    }

    pub fn set_title(&mut self, title: &str) {
        if let Some(engine) = self.engine.as_mut() {
            engine.set_title(title)
        }
    }
}

pub fn run(title: &str, width: f64, height: f64) -> Result<App, Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let attributes = Window::default_attributes()
        .with_title(title)
        .with_inner_size(winit::dpi::PhysicalSize::new(width, height));

    let mut app = App {
        engine: None,
        window_attributes: Some(attributes),
    };

    event_loop.run_app(&mut app)?;

    Ok(app)
}
