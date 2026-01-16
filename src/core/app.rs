use std::error::Error;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::core::engine::Engine;

pub(crate) struct App {
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
                let engine_future = Engine::new(window);
                let engine = engine_future;
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
        window_id: WindowId,
        event: WindowEvent
    ) {
        let engine = match self.engine.as_mut() {
            Some(engine) => engine,
            None => return,
        };
        match event {
            WindowEvent::Resized(new_size) => {engine.resize(new_size);},
            WindowEvent::CloseRequested => {event_loop.exit()},
            WindowEvent::RedrawRequested => {
                engine.update();
                engine.draw();
                engine.request_redraw();
            },

            _ => {},
        }
    }

}

pub fn run(title: &str, width: f64, height: f64) -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let attributes = Window::default_attributes()
        .with_title(title)
        .with_inner_size(PhysicalSize::new(width, height));

    let mut app = App {
        engine: None,
        window_attributes: Some(attributes),
    };

    event_loop.run_app(&mut app)?;

    Ok(())
}
