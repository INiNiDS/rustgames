/// demo_stress.rs - Stress test with thousands of sprites
/// 
/// This example demonstrates:
/// - Rendering many sprites (simulated for testing infrastructure)
/// - FPS counter with entity count
/// - Performance monitoring
/// - Trauma-based camera shake under load

use rustgames::prelude::*;
use rustgames::core::{app, FpsCounter};
use rustgames::graphics::{SpriteAnimation, AnimationMode};
use rustgames::window::KeyCode;
use glam::Vec2;
use rand::Rng;

struct StressDemo {
    animations: Vec<SpriteAnimation>,
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    fps_counter: FpsCounter,
    entity_count: usize,
    time: f32,
    shake_timer: f32,
}

impl Game for StressDemo {
    fn init(&mut self, engine: &mut Engine) {
        println!("=== Stress Test Demo ===");
        println!("Performance test with {} entities", self.entity_count);
        println!();
        println!("Features:");
        println!("  - {} animated sprites", self.entity_count);
        println!("  - Physics simulation (bouncing)");
        println!("  - Trauma-based camera shake");
        println!("  - Real-time FPS monitoring");
        println!();
        println!("Controls:");
        println!("  SPACE - Trigger camera shake");
        println!("  UP    - Add 1000 entities");
        println!("  DOWN  - Remove 1000 entities");
        println!("  ESC   - Exit");
        println!();
        
        // Load texture
        engine.get_texture_controller().load_texture(
            include_bytes!("../src/mistral.png"),
            "stress_sprite"
        );
        
        // Set up camera
        let camera = engine.get_camera_controller();
        camera.set_zoom(400.0);
        
        // Initialize entities
        self.spawn_entities(self.entity_count);
        
        println!("✓ Demo initialized with {} entities", self.entity_count);
    }

    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();
        
        // Update FPS counter
        self.fps_counter.update(delta);
        self.time += delta;
        
        // Auto-shake every 3 seconds
        self.shake_timer += delta;
        if self.shake_timer >= 3.0 {
            self.shake_timer = 0.0;
            engine.get_camera_controller().add_trauma(0.4);
        }
        
        // Update all animations
        for anim in &mut self.animations {
            anim.update(delta);
        }
        
        // Update positions (bouncing simulation)
        let bounds = 300.0;
        for i in 0..self.positions.len() {
            self.positions[i] += self.velocities[i] * delta;
            
            // Bounce off boundaries
            if self.positions[i].x.abs() > bounds {
                self.velocities[i].x *= -1.0;
                self.positions[i].x = self.positions[i].x.clamp(-bounds, bounds);
            }
            if self.positions[i].y.abs() > bounds {
                self.velocities[i].y *= -1.0;
                self.positions[i].y = self.positions[i].y.clamp(-bounds, bounds);
            }
        }
        
        // Handle controls
        let space_pressed = engine.get_event_queue().was_key_just_pressed(KeyCode::Space);
        let up_pressed = engine.get_event_queue().was_key_just_pressed(KeyCode::ArrowUp);
        let down_pressed = engine.get_event_queue().was_key_just_pressed(KeyCode::ArrowDown);
        let escape_pressed = engine.get_event_queue().was_key_just_pressed(KeyCode::Escape);
        
        if space_pressed {
            engine.get_camera_controller().add_trauma(0.8);
            println!("Manual shake triggered!");
        }
        
        if up_pressed {
            self.spawn_entities(1000);
            println!("Added 1000 entities. Total: {}", self.entity_count);
        }
        
        if down_pressed && self.entity_count > 1000 {
            self.remove_entities(1000);
            println!("Removed 1000 entities. Total: {}", self.entity_count);
        }
        
        if escape_pressed {
            self.print_final_stats();
            std::process::exit(0);
        }
        
        // Render sprites (using existing API - one at a time for now)
        // In a production instanced renderer, this would be a single draw call
        let texture_controller = engine.get_texture_controller();
        
        // For demonstration, we'll render a subset to avoid overwhelming the current renderer
        let render_count = self.entity_count.min(100);
        for i in 0..render_count {
            texture_controller.use_texture(
                "stress_sprite",
                Vec2::new(20.0, 20.0),
                self.positions[i],
            );
        }
        
        // Update window title
        if self.time >= 0.1 {
            self.time = 0.0;
            let title = format!(
                "Stress Test | Entities: {} | FPS: {:.0} | Frame: {:.1}ms | Min: {:.0} | Max: {:.0}",
                self.entity_count,
                self.fps_counter.fps(),
                self.fps_counter.frame_time_ms(),
                self.fps_counter.min_fps(),
                self.fps_counter.max_fps()
            );
            engine.set_title(&title);
        }
    }
}

impl StressDemo {
    fn spawn_entities(&mut self, count: usize) {
        let mut rng = rand::rng();
        
        for _ in 0..count {
            // Create animation
            let anim = SpriteAnimation::from_grid(
                2, 2, 4,
                5.0 + rng.random::<f32>() * 10.0, // Random FPS
                AnimationMode::Loop
            );
            self.animations.push(anim);
            
            // Random position
            let pos = Vec2::new(
                (rng.random::<f32>() - 0.5) * 600.0,
                (rng.random::<f32>() - 0.5) * 600.0,
            );
            self.positions.push(pos);
            
            // Random velocity
            let vel = Vec2::new(
                (rng.random::<f32>() - 0.5) * 100.0,
                (rng.random::<f32>() - 0.5) * 100.0,
            );
            self.velocities.push(vel);
        }
        
        self.entity_count = self.animations.len();
    }
    
    fn remove_entities(&mut self, count: usize) {
        let remove = count.min(self.animations.len());
        
        for _ in 0..remove {
            self.animations.pop();
            self.positions.pop();
            self.velocities.pop();
        }
        
        self.entity_count = self.animations.len();
    }
    
    fn print_final_stats(&self) {
        println!();
        println!("=== Final Statistics ===");
        println!("Total entities: {}", self.entity_count);
        println!("Average FPS: {:.1}", self.fps_counter.fps());
        println!("Average frame time: {:.2}ms", self.fps_counter.frame_time_ms());
        println!("Min FPS: {:.1}", self.fps_counter.min_fps());
        println!("Max FPS: {:.1}", self.fps_counter.max_fps());
        println!();
        println!("Note: Full instanced rendering would significantly improve these numbers!");
    }
}

fn main() {
    let initial_entities = 10000;
    
    let game = StressDemo {
        animations: Vec::with_capacity(initial_entities),
        positions: Vec::with_capacity(initial_entities),
        velocities: Vec::with_capacity(initial_entities),
        fps_counter: FpsCounter::new(),
        entity_count: 0,
        time: 0.0,
        shake_timer: 0.0,
    };
    
    println!("Starting stress test demo with {} entities...", initial_entities);
    println!();
    
    app::run("Demo: Stress Test", 1280.0, 720.0, Box::new(game))
        .expect("Failed to run stress test");
}
