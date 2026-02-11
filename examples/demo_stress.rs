use glam::{Vec2, Vec4};
use rand::Rng;
use rustgames::core::app;
use rustgames::prelude::*;

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

        engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/mistral.png"), "stress_sprite");

        let camera = engine.get_camera();
        camera.set_zoom(1.0);

        self.spawn_entities(self.entity_count);

        println!("✓ Demo initialized with {} entities", self.entity_count);

        engine.get_audio_system().load_sound(
            "perdej",
            "/home/ininids/RustroverProjects/rsgames/src/sound_03850.mp3",
        );
    }

    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();

        self.fps_counter.update(delta);
        self.time += delta;

        self.shake_timer += delta;
        if self.shake_timer >= 3.0 {
            self.shake_timer = 0.0;
            engine.get_camera().add_trauma(0.4);
        }

        for anim in &mut self.animations {
            anim.update(delta);
        }

        let bounds = 300.0;
        for i in 0..self.positions.len() {
            self.positions[i] += self.velocities[i] * delta;

            if self.positions[i].x.abs() > bounds {
                self.velocities[i].x *= -1.0;
                self.positions[i].x = self.positions[i].x.clamp(-bounds, bounds);
            }
            if self.positions[i].y.abs() > bounds {
                self.velocities[i].y *= -1.0;
                self.positions[i].y = self.positions[i].y.clamp(-bounds, bounds);
            }
        }

        let texture_controller = engine.get_texture_controller();

        for i in 0..self.entity_count {
            let uv = self.animations[i].current_uv();

            let instance =
                SpriteInstance::new(self.positions[i], Vec2::new(20.0, 20.0), 0.0, uv, Vec4::ONE);

            texture_controller.add_instance("stress_sprite", instance);
        }

        if self.time >= 0.1 {
            self.time = 0.0;
            let title = format!(
                "Stress Test | Entities: {} (ALL RENDERED) | FPS: {:.0} | Frame: {:.1}ms | Min: {:.0} | Max: {:.0}",
                self.entity_count,
                self.fps_counter.fps(),
                self.fps_counter.frame_time_ms(),
                self.fps_counter.min_fps(),
                self.fps_counter.max_fps()
            );
            engine.set_title(&title);
        }
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        let space_pressed = engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Space);
        let up_pressed = engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::ArrowUp);
        let down_pressed = engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::ArrowDown);
        let escape_pressed = engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Escape);

        if space_pressed {
            engine.get_camera().add_trauma(0.8);
            let _ = engine.get_audio_system().play("perdej");
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
    }
}

impl StressDemo {
    fn spawn_entities(&mut self, count: usize) {
        let mut rng = rand::rng();

        for _ in 0..count {
            let anim = SpriteAnimation::from_grid(
                2,
                2,
                4,
                5.0 + rng.random::<f32>() * 10.0,
                AnimationMode::Loop,
            );
            self.animations.push(anim);

            let pos = Vec2::new(
                (rng.random::<f32>() - 0.5) * 600.0,
                (rng.random::<f32>() - 0.5) * 600.0,
            );
            self.positions.push(pos);

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
        println!(
            "Average frame time: {:.2}ms",
            self.fps_counter.frame_time_ms()
        );
        println!("Min FPS: {:.1}", self.fps_counter.min_fps());
        println!("Max FPS: {:.1}", self.fps_counter.max_fps());
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

    println!("Starting stress test demo with {initial_entities} entities...");
    println!();

    let window_config = WindowConfig {
        title: "Demo: Stress Test".to_string(),
        width: 2560,
        height: 1440,
        resizable: false,
        fullscreen: true,
        vsync: true,
        background_color: Color::WHITE,
    };

    app::run(window_config, Box::new(game)).expect("Failed to run stress test");
}
