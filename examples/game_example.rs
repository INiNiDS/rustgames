use glam::Vec2;
use rustgames::core::app;
use rustgames::graphics::VfxRenderer;
use rustgames::prelude::*;

struct EffectsDemo {
    renderer_alpha: VfxRenderer,
    fps_counter: FpsCounter,
    anim_controller: AnimationSystem,
    base_state: VisualState,
    time: f32,
    texture_size: Vec2,
}

impl Game for EffectsDemo {
    fn init(&mut self, engine: &mut Engine) {
        println!("=== Effects System Demo (renderer_alpha) ===");
        println!("Controls:");
        println!("  1 - Flash effect (white)");
        println!("  2 - Screen shake");
        println!("  3 - Particle sparkles");
        println!("  4 - Color overlay toggle");
        println!("  5 - Particle explosion");
        println!("  SPACE - Clear all effects");
        println!("  ENTER - Entrance animation");
        println!("  ESC   - Exit");

        engine.get_texture_controller().load_texture(
            include_bytes!("../src/mistral.png"),
            "sprite",
        );

        engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/OIP-475081084.jpg"), "background");

        engine.get_camera().set_zoom(1.0);

        let entrance = TimelineBuilder::new()
            .parallel(vec![
                (
                    Animation::SlideIn {
                        from: Direction::Bottom,
                        distance: 0.8,
                        duration: 1.0,
                    },
                    Easing::EaseOut,
                ),
                (
                    Animation::Scale {
                        from: 0.0,
                        to: 1.0,
                        duration: 1.0,
                    },
                    Easing::Elastic,
                ),
                (Animation::FadeIn { duration: 0.8 }, Easing::Linear),
            ])
            .build();

        self.anim_controller.start_timeline(entrance);
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps_counter.update(dt);
        self.time += dt;
        self.anim_controller.update(dt);
        self.renderer_alpha.update(dt);

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Digit1)
        {
            self.renderer_alpha.add_effect(VfxEffect::Flash {
                color: Color::WHITE,
                duration: 0.5,
            });
            println!("Flash!");
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Digit2)
        {
            engine.get_camera().add_trauma(0.6);
            println!("Shake!");
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Digit3)
        {
            self.renderer_alpha
                .add_effect(VfxEffect::Emitter(EmitterConfig::sparkles(
                    Vec2::ZERO,
                )));
            println!("Sparkles!");
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Digit4)
        {
            if self.renderer_alpha.overlay_state().active {
                self.renderer_alpha.clear_overlay();
                println!("Overlay removed");
            } else {
                self.renderer_alpha
                    .add_effect(VfxEffect::Overlay {
                        color: Color::BLUE,
                        alpha: 0.25,
                    });
                println!("Blue overlay applied");
            }
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Digit5)
        {
            self.renderer_alpha
                .add_effect(VfxEffect::Emitter(EmitterConfig::explosion(
                    Vec2::ZERO,
                )));
            println!("Explosion!");
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Space)
        {
            self.renderer_alpha.clear_all();
            println!("All effects cleared");
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
            self.anim_controller.start_timeline(pulse);
            println!("Pulse animation!");
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Escape)
        {
            std::process::exit(0);
        }

        let tc = engine.get_texture_controller();

        let sprite_size = tc
            .get_texture("sprite")
            .map_or(self.texture_size, |tex| {
                self.texture_size = tex.size;
                self.texture_size
            });

        tc.use_texture("background", Vec2::new(2560.0, 1440.0), Vec2::ZERO, 0.0, 1.0);

        let visual = self
            .anim_controller
            .evaluate(self.base_state, sprite_size, None);

        tc.use_texture(
            "sprite",
            sprite_size * visual.scale,
            visual.position,
            visual.rotation,
            visual.opacity,
        );

        let frame = self.renderer_alpha.build_frame();

        for particle_inst in &frame.particle {
            tc.add_instance("sprite", *particle_inst);
        }

        if self.time >= 0.5 {
            self.time = 0.0;
            let title = format!(
                "Effects Demo | FPS: {:.0} | Active Effects: {}",
                self.fps_counter.fps(),
                self.renderer_alpha.active_effect_count(),
            );
            engine.set_title(&title);
        }
    }
}

fn main() {
    let game = EffectsDemo {
        renderer_alpha: VfxRenderer::new(),
        fps_counter: FpsCounter::new(),
        anim_controller: AnimationSystem::new(),
        base_state: VisualState::default(),
        time: 0.0,
        texture_size: Vec2::new(128.0, 128.0),
    };

    let window_config = WindowConfig {
        title: "Effects System Demo (renderer_alpha)".to_string(),
        width: 1280,
        height: 720,
        resizable: true,
        fullscreen: false,
        vsync: true,
        background_color: Color::BLACK,
    };

    app::run(window_config, Box::new(game)).expect("Failed to run effects demo");
}
