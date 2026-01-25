use winit::event::{ElementState, MouseButton as WinitMouseButton, WindowEvent as WinitWindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Mouse button enum
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

/// Engine event types
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

/// Event handler trait for processing events
pub trait EventHandler {
    fn on_key_pressed(&mut self, _key: KeyCode) {}
    fn on_key_released(&mut self, _key: KeyCode) {}
    fn on_mouse_moved(&mut self, _x: f32, _y: f32) {}
    fn on_mouse_pressed(&mut self, _button: MouseButton) {}
    fn on_mouse_released(&mut self, _button: MouseButton) {}
    fn on_mouse_wheel(&mut self, _delta: f32) {}
    fn on_window_resized(&mut self, _width: u32, _height: u32) {}
    fn on_window_focused(&mut self, _focused: bool) {}
}

/// Event queue for collecting and processing events
pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    /// Add an event to the queue
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }
    
    /// Get all events and clear the queue
    pub fn drain(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }
    
    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
    
    /// Clear all events
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert winit events to engine events
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
