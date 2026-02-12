use glam::Vec2;
use rustgames::core::app;
use rustgames::graphics::effects::{Animation, Direction, Easing, TimelineBuilder, VisualState};
use rustgames::prelude::*;

struct SingleDemo {
    frame_animation: SpriteAnimation,
    base_state: VisualState,
    fps_counter: FpsCounter,
    time: f32,
    shake_cooldown: f32,
    texture_size: Vec2,
}

impl Game for SingleDemo {
    fn init(&mut self, engine: &mut Engine) {
        println!("=== Single Animated Sprite Demo ===");
        println!("Controls:");
        println!("  SPACE - Trigger Camera Shake");
        println!("  ENTER - Trigger 'Pulse' Animation");
        println!("  ESC   - Exit");

        engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/mistral.png"), "demo_sprite");

        engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/OIP-475081084.jpg"), "background");

        let camera = engine.get_camera();
        camera.set_zoom(1.0);

        self.base_state = VisualState {
            position: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 45.0_f32.to_radians(),
            ..Default::default()
        };

        let entrance_timeline = TimelineBuilder::new()
            .parallel(vec![
                (
                    Animation::SlideIn {
                        from: Direction::Bottom,
                        distance: 0.8,
                        duration: 1.2,
                    },
                    Easing::EaseOut,
                ),
                (
                    Animation::Scale {
                        from: 0.0,
                        to: 1.0,
                        duration: 1.2,
                    },
                    Easing::Elastic,
                ),
                (Animation::FadeIn { duration: 1.0 }, Easing::Linear),
            ])
            .build();

        engine
            .get_animation_system()
            .start_timeline(entrance_timeline);

        println!("✓ Demo initialized. Sprite appearing...");
    }

    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();

        self.fps_counter.update(delta);
        self.time += delta;
        if self.shake_cooldown > 0.0 {
            self.shake_cooldown -= delta;
        }

        self.frame_animation.update(delta);
        engine.get_animation_system().update(delta);

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Escape)
        {
            self.print_final_stats();
            std::process::exit(0);
        }

        let sprite_size = engine
            .get_texture_controller()
            .get_texture("demo_sprite")
            .map_or(self.texture_size, |tex| {
                self.texture_size = tex.size;
                self.texture_size
            });

        let window_width = 2560.0;
        let window_height = 1440.0;
        let camera_zoom = 1.0;

        let bg_world_size = Vec2::new(window_width / camera_zoom, window_height / camera_zoom);

        engine.get_texture_controller().use_texture(
            "background",
            bg_world_size,
            Vec2::ZERO,
            0.0_f32.to_radians(),
            1.0,
        );

        let visual = engine
            .get_animation_system()
            .evaluate(self.base_state, sprite_size, None);

        engine.get_texture_controller().use_texture(
            "demo_sprite",
            sprite_size * visual.scale,
            visual.position,
            visual.rotation,
            visual.opacity,
        );

        if self.time >= 0.5 {
            self.time = 0.0;
            let title = format!(
                "FPS: {:.0} | Anim Active: {}",
                self.fps_counter.fps(),
                engine.get_animation_system().is_playing()
            );
            engine.set_title(&title);
        }
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Space)
            && self.shake_cooldown <= 0.0
        {
            engine.get_camera().add_trauma(0.5);
            println!("Camera shake triggered!");
            self.shake_cooldown = 0.5;
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Enter)
        {
            let pulse = TimelineBuilder::new()
                .single(
                    Animation::Scale {
                        from: 1.0,
                        to: 1.5,
                        duration: 0.2,
                    },
                    Easing::EaseOut,
                )
                .single(
                    Animation::Scale {
                        from: 1.5,
                        to: 1.0,
                        duration: 0.4,
                    },
                    Easing::Bounce,
                )
                .build();

            engine.get_animation_system().start_timeline(pulse);
            println!("Pulse animation triggered!");
        }
    }
}

impl SingleDemo {
    fn print_final_stats(&self) {
        println!();
        println!("=== Final Statistics ===");
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
    let animation = SpriteAnimation::from_grid(2, 2, 4, 8.0, AnimationMode::Loop);

    let game = SingleDemo {
        frame_animation: animation,
        base_state: VisualState::default(),
        fps_counter: FpsCounter::new(),
        time: 0.0,
        shake_cooldown: 0.0,
        texture_size: Vec2::ZERO,
    };

    let window_config = WindowConfig {
        title: "Demo: Animated Sprite System".to_string(),
        width: 2560,
        height: 1440,
        resizable: false,
        fullscreen: true,
        vsync: true,
        background_color: Color::BLACK,
    };

    app::run(window_config, Box::new(game)).expect("Failed to run demo");
}
