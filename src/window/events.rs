use winit::event::{
    ElementState, MouseButton as WinitMouseButton, WindowEvent as WinitWindowEvent,
};
pub use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;
use wgpu::naga::FastHashSet;

/// A mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Primary (left) mouse button.
    Left,
    /// Secondary (right) mouse button.
    Right,
    /// Middle mouse button (scroll wheel click).
    Middle,
    /// Any other button, identified by its raw index.
    Other(u16),
}

impl From<WinitMouseButton> for MouseButton {
    fn from(button: WinitMouseButton) -> Self {
        match button {
            WinitMouseButton::Left => Self::Left,
            WinitMouseButton::Right => Self::Right,
            WinitMouseButton::Middle => Self::Middle,
            WinitMouseButton::Back => Self::Other(3),
            WinitMouseButton::Forward => Self::Other(4),
            WinitMouseButton::Other(id) => Self::Other(id),
        }
    }
}

/// An input or window event consumed by the game loop.
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// The window was resized to the given `(width, height)` in physical pixels.
    WindowResized(u32, u32),
    /// The user requested to close the window.
    WindowClosed,
    /// The window gained or lost focus (`true` = gained).
    WindowFocused(bool),
    /// A keyboard key was pressed.
    KeyPressed(KeyCode),
    /// A keyboard key was released.
    KeyReleased(KeyCode),
    /// The cursor moved to `(x, y)` in physical pixels.
    MouseMoved(f64, f64),
    /// A mouse button was pressed.
    MousePressed(MouseButton),
    /// A mouse button was released.
    MouseReleased(MouseButton),
    /// The scroll wheel moved by `delta` lines (positive = up).
    MouseWheel(f32),
}

/// Trait for receiving individual event callbacks from the engine.
pub trait EventHandler {
    /// Called when `key` transitions from released to pressed.
    fn on_key_pressed(&mut self, _key: KeyCode) {}
    /// Called when `key` transitions from pressed to released.
    fn on_key_released(&mut self, _key: KeyCode) {}
    /// Called when the cursor moves to `(x, y)` in physical pixels.
    fn on_mouse_moved(&mut self, _x: f64, _y: f64) {}
    /// Called when a mouse button is pressed.
    fn on_mouse_pressed(&mut self, _button: MouseButton) {}
    /// Called when a mouse button is released.
    fn on_mouse_released(&mut self, _button: MouseButton) {}
    /// Called when the scroll wheel moves; `delta` is positive for scroll-up.
    fn on_mouse_wheel(&mut self, _delta: f32) {}
    /// Called when the window is resized to `(width, height)` physical pixels.
    fn on_window_resized(&mut self, _width: u32, _height: u32) {}
    /// Called when the window gains (`true`) or loses (`false`) focus.
    fn on_window_focused(&mut self, _focused: bool) {}
    /// Called when the user requests the window to close.
    fn on_window_closed(&mut self) {}
}

/// Buffers events for the current frame and tracks held-down keys.
pub struct EventQueue {
    events: Vec<Event>,
    pressed_keys: FastHashSet<KeyCode>,
}

impl EventQueue {
    /// Creates a new, empty [`EventQueue`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            pressed_keys: FastHashSet::default(),
        }
    }

    /// Appends `event` to the queue. `KeyPressed` / `KeyReleased` events also
    /// update the held-key tracker.
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

    /// Drains and returns all events queued since the last drain.
    pub fn drain(&mut self) -> Vec<Event> {
        std::mem::take(&mut self.events)
    }

    /// Returns `true` when no events are queued for this frame.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Discards all queued events without returning them.
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Returns `true` while `key` is held down (pressed and not yet released).
    #[must_use]
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Returns `true` while `key` is held down (alias for
    /// [`is_key_pressed`][Self::is_key_pressed]).
    #[must_use]
    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Returns `true` if `key` transitioned to pressed during this frame's
    /// event batch.
    #[must_use]
    pub fn was_key_just_pressed(&self, key: KeyCode) -> bool {
        self.events
            .iter()
            .any(|e| matches!(e, Event::KeyPressed(k) if *k == key))
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use]
pub fn convert_window_event(event: &WinitWindowEvent) -> Option<Event> {
    match event {
        WinitWindowEvent::Resized(_)
        | WinitWindowEvent::CloseRequested
        | WinitWindowEvent::Focused(_) => convert_window_lifecycle_event(event),
        _ => convert_input_event(event),
    }
}

const fn convert_window_lifecycle_event(event: &WinitWindowEvent) -> Option<Event> {
    match event {
        WinitWindowEvent::Resized(size) => Some(Event::WindowResized(size.width, size.height)),
        WinitWindowEvent::CloseRequested => Some(Event::WindowClosed),
        WinitWindowEvent::Focused(focused) => Some(Event::WindowFocused(*focused)),
        _ => None,
    }
}

fn convert_input_event(event: &WinitWindowEvent) -> Option<Event> {
    match event {
        WinitWindowEvent::KeyboardInput { event, .. } => convert_keyboard_event(event),
        WinitWindowEvent::CursorMoved { position, .. } => {
            Some(Event::MouseMoved(position.x, position.y))
        }
        WinitWindowEvent::MouseInput { state, button, .. } => {
            Some(convert_mouse_button_event(*state, *button))
        }
        WinitWindowEvent::MouseWheel { delta, .. } => Some(convert_mouse_wheel_event(delta)),
        _ => None,
    }
}

const fn convert_keyboard_event(event: &winit::event::KeyEvent) -> Option<Event> {
    if let PhysicalKey::Code(keycode) = event.physical_key {
        match event.state {
            ElementState::Pressed => Some(Event::KeyPressed(keycode)),
            ElementState::Released => Some(Event::KeyReleased(keycode)),
        }
    } else {
        None
    }
}

fn convert_mouse_button_event(state: ElementState, button: WinitMouseButton) -> Event {
    let button = MouseButton::from(button);
    match state {
        ElementState::Pressed => Event::MousePressed(button),
        ElementState::Released => Event::MouseReleased(button),
    }
}


#[allow(clippy::cast_possible_truncation)]
fn convert_mouse_wheel_event(delta: &winit::event::MouseScrollDelta) -> Event {
    let delta_y = match delta {
        winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
        winit::event::MouseScrollDelta::PixelDelta(pos) => {
            (pos.y / 100.0) as f32
        },
    };
    Event::MouseWheel(delta_y)
}
