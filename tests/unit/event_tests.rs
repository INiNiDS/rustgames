use rustgames::window::{Event, EventQueue, WindowConfig};

#[test]
fn key_tracking() {
    let mut queue = EventQueue::new();
    queue.push(Event::KeyPressed(winit::keyboard::KeyCode::KeyW));
    assert!(queue.is_key_pressed(winit::keyboard::KeyCode::KeyW));
    assert!(queue.was_key_just_pressed(winit::keyboard::KeyCode::KeyW));
    queue.push(Event::KeyReleased(winit::keyboard::KeyCode::KeyW));
    assert!(!queue.is_key_pressed(winit::keyboard::KeyCode::KeyW));
}

#[test]
fn drain_empties_queue() {
    let mut queue = EventQueue::new();
    queue.push(Event::WindowClosed);
    queue.push(Event::WindowFocused(true));
    let events = queue.drain();
    assert_eq!(events.len(), 2);
    assert!(queue.is_empty());
}

#[test]
fn window_config_default() {
    let config = WindowConfig::default();
    assert_eq!(config.width, 800);
    assert_eq!(config.height, 600);
    assert!(config.resizable);
    assert!(config.vsync);
}

#[test]
fn window_config_new() {
    let config = WindowConfig::new("My Game", 1920, 1080);
    assert_eq!(config.title, "My Game");
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
}
