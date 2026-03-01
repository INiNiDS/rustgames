use crate::core::Game;
use crate::core::engine::Engine;
use crate::window::WindowConfig;
use crate::window::convert_window_event;
use std::error::Error;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

pub struct App {
    engine: Option<Engine>,
    window_attributes: Option<WindowAttributes>,
    game: Box<dyn Game>,
    window_config: WindowConfig,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = self.window_attributes.take().unwrap_or_default();
        let window_result = event_loop.create_window(attributes);
        match window_result {
            Ok(window) => {
                let window = Arc::new(window);
                match Engine::new(window) {
                    Ok(mut engine) => {
                        engine.set_window_config(&self.window_config);
                        self.game.init(&mut engine);
                        self.engine = Some(engine);
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        event_loop.exit();
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to create window: {e:?}");
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };

        if let Some(ev) = convert_window_event(&event) {
            engine.push_event(ev);
        }

        match event {
            WindowEvent::Resized(new_size) => {
                engine.resize(new_size);
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.redraw();
            }
            _ => {}
        }
    }
}

impl App {
    fn redraw(&mut self) {
        if let Some(engine) = self.engine.as_mut() {
            self.game.handle_update(engine);
            engine.update();
            self.game.update(engine);
            engine.draw();
            engine.request_redraw();
        }
    }
}

/// Creates a window and enters the main event loop. Returns when the window
/// is closed.
///
/// # Errors
///
/// Returns an error if the event loop cannot be created or exits with an error.
pub fn run(window_config: WindowConfig, game: Box<dyn Game>) -> Result<App, Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let attributes = Window::default_attributes()
        .with_title(window_config.title.clone())
        .with_inner_size(winit::dpi::PhysicalSize::new(
            window_config.width,
            window_config.height,
        ));

    let mut app = App {
        engine: None,
        window_attributes: Some(attributes),
        game,
        window_config,
    };

    event_loop.run_app(&mut app)?;

    Ok(app)
}
