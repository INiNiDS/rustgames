/// demo_single.rs - Demonstrates sprite animation and advanced camera features
/// 
/// This example shows:
/// - Creating animated sprites
/// - Trauma-based camera shake
/// - Smooth camera movement
/// - FPS counter for performance monitoring

use rustgames::prelude::*;
use rustgames::core::{app, FpsCounter};
use rustgames::graphics::{SpriteAnimation, AnimationMode};
use rustgames::window::KeyCode;
use glam::Vec2;

struct SingleDemo {
    animation: SpriteAnimation,
    fps_counter: FpsCounter,
    time: f32,
    shake_cooldown: f32,
}

impl Game for SingleDemo {
    fn init(&mut self, engine: &mut Engine) {
        println!("=== Single Animated Sprite Demo ===");
        println!("This demo shows sprite animation with advanced camera features");
        println!();
        println!("Features demonstrated:");
        println!("  - Sprite animation system (Loop mode)");
        println!("  - Trauma-based camera shake");
        println!("  - FPS counter");
        println!();
        println!("Controls:");
        println!("  SPACE - Trigger camera shake");
        println!("  ESC   - Exit");
        println!();
        
        // Load a sample texture
        engine.get_texture_controller().load_texture(
            include_bytes!("../src/mistral.png"),
            "demo_sprite"
        );
        
        // Set up camera with smooth zoom
        let camera = engine.get_camera_controller();
        camera.set_zoom(200.0);
        
        // Add a small initial shake to demonstrate the feature
        camera.add_trauma(0.3);
        
        println!("✓ Demo initialized. Texture loaded, animation running.");
        println!("✓ Camera shake triggered (trauma: 0.3)");
    }

    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();
        
        // Update FPS counter
        self.fps_counter.update(delta);
        self.time += delta;
        
        // Update animation
        self.animation.update(delta);
        
        // Update shake cooldown
        if self.shake_cooldown > 0.0 {
            self.shake_cooldown -= delta;
        }
        
        // Handle space for manual shake trigger
        if engine.get_event_queue().was_key_just_pressed(KeyCode::Space) && self.shake_cooldown <= 0.0 {
            let camera = engine.get_camera_controller();
            camera.add_trauma(0.5);
            println!("Camera shake triggered! Trauma: 0.5");
            self.shake_cooldown = 0.5; // Prevent spam
        }
        
        // Handle ESC to exit
        if engine.get_event_queue().was_key_just_pressed(KeyCode::Escape) {
            println!();
            println!("Demo finished. Final stats:");
            println!("  Average FPS: {:.1}", self.fps_counter.fps());
            println!("  Frame time: {:.2}ms", self.fps_counter.frame_time_ms());
            std::process::exit(0);
        }
        
        // Display the sprite using the existing API
        let texture_controller = engine.get_texture_controller();
        texture_controller.use_texture(
            "demo_sprite",
            Vec2::new(200.0, 200.0), // Size
            Vec2::ZERO, // Position (center)
        );
        
        // Update window title with FPS
        if self.time >= 0.5 {
            self.time = 0.0;
            let title = format!(
                "Demo: Single Sprite | FPS: {:.0} | Frame: {:.1}ms",
                self.fps_counter.fps(),
                self.fps_counter.frame_time_ms()
            );
            engine.set_title(&title);
        }
    }
}

fn main() {
    // Create animation (2x2 grid, 4 frames, 8 FPS, Loop mode)
    // This demonstrates the UV-based animation system
    let animation = SpriteAnimation::from_grid(2, 2, 4, 8.0, AnimationMode::Loop);
    
    let game = SingleDemo {
        animation,
        fps_counter: FpsCounter::new(),
        time: 0.0,
        shake_cooldown: 0.0,
    };
    
    println!("Starting single sprite demo...");
    println!();
    
    // Run the game
    app::run("Demo: Single Animated Sprite", 800.0, 600.0, Box::new(game))
        .expect("Failed to run demo");
}
