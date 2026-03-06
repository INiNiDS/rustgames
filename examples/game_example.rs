use glam::Vec2;
use rustgames::core::app;
use rustgames::graphics::VfxRenderer;
use rustgames::prelude::*;

struct EffectsDemo {
    renderer_alpha: VfxRenderer,
    fps_counter: FpsCounter,
    base_state: VisualState,
    time: f32,
    texture_size: Vec2,
    text_id: usize,
    is_aggressive: bool,
    is_complete: bool,
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

        if let Err(e) = engine.get_texture_controller().load_texture(
            include_bytes!("../src/static/textures/OIP-475081084.jpg"),
            "background",
        ) { eprintln!("{e}"); }

        if let Err(e) = engine.get_texture_controller().load_texture(
            include_bytes!("../src/static/textures/mistral.png"),
            "sprite",
        ) { eprintln!("{e}"); }

        engine.get_camera().set_zoom(1.0);

        self.text_id = engine.get_text_system().add_text(
            "Hello! I'm Luna! How are you?",
            TextSpeed::Slow,
            50.0,
            50.0,
            TextStyle::new(64.0).with_color(Color::YELLOW),
            PunctuationConfig::DEFAULT,
        );

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

        engine.get_animation_system().start_timeline(entrance);
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps_counter.update(dt);
        self.update_scene(engine);
    }
    fn handle_update(&mut self, engine: &mut Engine) {
        self.handle_input(engine);
    }
}

impl EffectsDemo {
    fn handle_input(&mut self, engine: &mut Engine) {
        let eq = engine.get_event_queue();
        let k1 = eq.was_key_just_pressed(KeyCode::Digit1);
        let k2 = eq.was_key_just_pressed(KeyCode::Digit2);
        let k3 = eq.was_key_just_pressed(KeyCode::Digit3);
        let k4 = eq.was_key_just_pressed(KeyCode::Digit4);
        let k5 = eq.was_key_just_pressed(KeyCode::Digit5);
        let space = eq.was_key_just_pressed(KeyCode::Space);
        let enter = eq.was_key_just_pressed(KeyCode::Enter);
        let esc = eq.was_key_just_pressed(KeyCode::Escape);

        if k1 { self.on_flash(engine); }
        if k2 { self.on_shake(engine); }
        if k3 { self.on_sparkles(engine); }
        if k4 { self.on_overlay_toggle(engine); }
        if k5 { self.on_explosion(engine); }
        if space { self.on_clear(engine); }
        if enter { self.on_pulse(engine); }
        if esc { std::process::exit(0); }
    }

    fn on_flash(&mut self, _engine: &mut Engine) {
        self.renderer_alpha.add_effect(VfxEffect::Flash {
            color: Color::WHITE,
            duration: 0.5,
        });
        println!("Flash!");
    }

    fn on_shake(&mut self, engine: &mut Engine) {
        engine.get_camera().add_trauma(0.6);
        println!("Shake!");
    }

    fn on_sparkles(&mut self, _engine: &mut Engine) {
        self.renderer_alpha
            .add_effect(VfxEffect::Emitter(EmitterConfig::sparkles(Vec2::ZERO)));
        println!("Sparkles!");
    }

    fn on_overlay_toggle(&mut self, _engine: &mut Engine) {
        if self.renderer_alpha.overlay_state().active {
            self.renderer_alpha.clear_overlay();
            println!("Overlay removed");
        } else {
            self.renderer_alpha.add_effect(VfxEffect::Overlay {
                color: Color::BLUE,
                alpha: 0.25,
            });
            println!("Blue overlay applied");
        }
    }

    fn on_explosion(&mut self, _engine: &mut Engine) {
        self.renderer_alpha
            .add_effect(VfxEffect::Emitter(EmitterConfig::explosion(Vec2::ZERO)));
        println!("Explosion!");
    }

    fn on_clear(&mut self, _engine: &mut Engine) {
        self.renderer_alpha.clear_all();
        println!("All effects cleared");
    }

    fn on_pulse(&mut self, engine: &mut Engine) {
        let pulse = Self::build_pulse_timeline();
        engine.get_animation_system().start_timeline(pulse);
        println!("Pulse animation!");
    }

    fn build_pulse_timeline() -> Vec<TimelineStep> {
        TimelineBuilder::new()
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
            .build()
    }

    fn update_scene(&mut self, engine: &mut Engine) {
        let sprite_size = engine
            .get_texture_controller()
            .get_texture("sprite")
            .map_or(self.texture_size, |tex| {
                self.texture_size = tex.size;
                self.texture_size
            });

        engine.get_texture_controller().use_texture(
            "background",
            Vec2::new(2560.0, 1440.0),
            Vec2::ZERO,
            0.0,
            1.0,
        );

        let visual = engine
            .get_animation_system()
            .evaluate(self.base_state, sprite_size, None);

        engine.get_texture_controller().use_texture(
            "sprite",
            sprite_size * visual.scale,
            visual.position,
            visual.rotation,
            visual.opacity,
        );

        let frame = self.renderer_alpha.build_frame();

        for particle_inst in &frame.particle {
            engine
                .get_texture_controller()
                .add_instance("sprite", *particle_inst);
        }

        let text_progress = engine.get_text_system().get_progress(self.text_id);

        if text_progress > 0.6 && !self.is_aggressive {
            self.is_aggressive = true;
            let _ = engine.get_text_system().set_text(
                self.text_id,
                "I'll kill you...",
                TextSpeed::Slow,
                TextStyle::new(64.0).with_color(Color::RED),
                PunctuationConfig::INSTANT,
            );
        } else if self.is_aggressive
            && engine.get_text_system().is_complete(self.text_id)
            && !self.is_complete
        {
            let _ = engine.get_text_system().set_text(
                self.text_id,
                "Hello! I'm Luna! How are you?",
                TextSpeed::Slow,
                TextStyle::new(64.0).with_color(Color::YELLOW),
                PunctuationConfig::default(),
            );
            let _ = engine.get_text_system().set_progress(self.text_id, 0.6);
            self.is_complete = true;
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
        base_state: VisualState::default(),
        time: 0.0,
        texture_size: Vec2::new(128.0, 128.0),
        text_id: 0,
        is_aggressive: false,
        is_complete: false,
    };

    let window_config = WindowConfig {
        title: "Effects System Demo (renderer_alpha)".to_string(),
        width: 2560,
        height: 1440,
        resizable: true,
        fullscreen: true,
        vsync: true,
        background_color: Color::BLACK,
        language: Language::resolve("en_us").unwrap(),
    };

    app::run(window_config, Box::new(game)).expect("Failed to run effects demo");
}
