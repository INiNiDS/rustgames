use winit::event::{ElementState, MouseButton as WinitMouseButton, WindowEvent as WinitWindowEvent};
pub use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

impl From<WinitMouseButton> for MouseButton {
    fn from(button: WinitMouseButton) -> Self {
        match button {
            WinitMouseButton::Left => MouseButton::Left,
            WinitMouseButton::Right => MouseButton::Right,
            WinitMouseButton::Middle => MouseButton::Middle,
            WinitMouseButton::Back => MouseButton::Other(3),
            WinitMouseButton::Forward => MouseButton::Other(4),
            WinitMouseButton::Other(id) => MouseButton::Other(id),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    WindowResized(u32, u32),
    WindowClosed,
    WindowFocused(bool),
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseMoved(f32, f32),
    MousePressed(MouseButton),
    MouseReleased(MouseButton),
    MouseWheel(f32),
}

pub trait EventHandler {
    fn on_key_pressed(&mut self, _key: KeyCode) {}
    fn on_key_released(&mut self, _key: KeyCode) {}
    fn on_mouse_moved(&mut self, _x: f32, _y: f32) {}
    fn on_mouse_pressed(&mut self, _button: MouseButton) {}
    fn on_mouse_released(&mut self, _button: MouseButton) {}
    fn on_mouse_wheel(&mut self, _delta: f32) {}
    fn on_window_resized(&mut self, _width: u32, _height: u32) {}
    fn on_window_focused(&mut self, _focused: bool) {}
    fn on_window_closed(&mut self) {}
}

pub struct EventQueue {
    events: Vec<Event>,
    pressed_keys: std::collections::HashSet<KeyCode>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            pressed_keys: std::collections::HashSet::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        match event {
            Event::KeyPressed(key) => {
                self.pressed_keys.insert(key);
            }
            Event::KeyReleased(key) => {
                self.pressed_keys.remove(&key);
            }
            _ => {}
        }
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn was_key_just_pressed(&self, key: KeyCode) -> bool {
        self.events.iter().any(|e| matches!(e, Event::KeyPressed(k) if *k == key))
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

pub fn convert_window_event(event: &WinitWindowEvent) -> Option<Event> {
    match event {
        WinitWindowEvent::Resized(size) => {
            Some(Event::WindowResized(size.width, size.height))
        }
        WinitWindowEvent::CloseRequested => {
            Some(Event::WindowClosed)
        }
        WinitWindowEvent::Focused(focused) => {
            Some(Event::WindowFocused(*focused))
        }
        WinitWindowEvent::KeyboardInput { event, .. } => {
            if let PhysicalKey::Code(keycode) = event.physical_key {
                match event.state {
                    ElementState::Pressed => Some(Event::KeyPressed(keycode)),
                    ElementState::Released => Some(Event::KeyReleased(keycode)),
                }
            } else {
                None
            }
        }
        WinitWindowEvent::CursorMoved { position, .. } => {
            Some(Event::MouseMoved(position.x as f32, position.y as f32))
        }
        WinitWindowEvent::MouseInput { state, button, .. } => {
            let button = MouseButton::from(*button);
            match state {
                ElementState::Pressed => Some(Event::MousePressed(button)),
                ElementState::Released => Some(Event::MouseReleased(button)),
            }
        }
        WinitWindowEvent::MouseWheel { delta, .. } => {
            let delta_y = match delta {
                winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                winit::event::MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 100.0,
            };
            Some(Event::MouseWheel(delta_y))
        }
        _ => None,
    }
}
