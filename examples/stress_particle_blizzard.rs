//! **Particle Blizzard Stress Test** — Data-Oriented (SoA) Edition.
//!
//! Engine systems under stress simultaneously:
//!   • 50 000+ bouncing physics particles (Structure of Arrays layout)
//!   • Per-particle `SpriteAnimation` cycling through 4 frames
//!   • `AnimationSystem` timeline (FadeIn → SlideIn loop) replayed every 3 s
//!   • `TypewriterEffect` burst: a new text line spawned every 0.5 s
//!   • Rolling `FpsCounter` (min / avg / max)
//!   • Camera trauma decaying automatically

use glam::{Vec2, Vec4};
use rand::RngExt;
use rustgames::core::app;
use rustgames::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const INITIAL_PARTICLES: usize = 50_000;
const BOUNDS: f32 = 500.0;
const MIN_SIZE: f32 = 4.0;
const MAX_SIZE: f32 = 10.0;
const GRAVITY: f32 = 60.0;

// ---------------------------------------------------------------------------
// Structure of Arrays (SoA) Particle Container
// ---------------------------------------------------------------------------

struct BlizzardSoA {
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
    sizes: Vec<f32>,
    anims: Vec<SpriteAnimation>,
    colors: Vec<Color>,
    spins: Vec<f32>,
    angles: Vec<f32>,
}

impl BlizzardSoA {
    fn new(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            velocities: Vec::with_capacity(capacity),
            sizes: Vec::with_capacity(capacity),
            anims: Vec::with_capacity(capacity),
            colors: Vec::with_capacity(capacity),
            spins: Vec::with_capacity(capacity),
            angles: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, rng: &mut impl rand::Rng) {
        let cols = [Color::CYAN, Color::WHITE, Color::BLUE, Color::LIGHT_GRAY];
        self.colors.push(cols[rng.random_range(0..cols.len())]);

        self.positions.push(Vec2::new(
            (rng.random::<f32>() - 0.5) * BOUNDS * 2.0,
            (rng.random::<f32>() - 0.5) * BOUNDS * 2.0,
        ));

        self.velocities.push(Vec2::new(
            (rng.random::<f32>() - 0.5) * 80.0,
            (rng.random::<f32>() - 0.5) * 80.0 - 20.0,
        ));

        self.sizes
            .push(MIN_SIZE + rng.random::<f32>() * (MAX_SIZE - MIN_SIZE));

        self.anims.push(SpriteAnimation::from_grid(
            2,
            2,
            4,
            4.0 + rng.random::<f32>() * 8.0,
            AnimationMode::Loop,
        ));

        self.spins.push((rng.random::<f32>() - 0.5) * 4.0);
        self.angles
            .push(rng.random::<f32>() * std::f32::consts::TAU);
    }

    fn truncate(&mut self, new_len: usize) {
        self.positions.truncate(new_len);
        self.velocities.truncate(new_len);
        self.sizes.truncate(new_len);
        self.anims.truncate(new_len);
        self.colors.truncate(new_len);
        self.spins.truncate(new_len);
        self.angles.truncate(new_len);
    }

    fn len(&self) -> usize {
        self.positions.len()
    }
}

// ---------------------------------------------------------------------------
// Text burst tracker
// ---------------------------------------------------------------------------

struct BurstLine {
    effect: TypewriterEffect,
    lifetime: f32,
}

// ---------------------------------------------------------------------------
// Main game state
// ---------------------------------------------------------------------------

struct ParticleBlizzard {
    soa: BlizzardSoA,
    fps_counter: FpsCounter,
    time: f32,
    title_timer: f32,
    text_timer: f32,
    text_enabled: bool,
    show_fps: bool,
    burst_lines: Vec<BurstLine>,
    timeline_timer: f32,
}

impl ParticleBlizzard {
    fn new() -> Self {
        let mut rng = rand::rng();
        let mut soa = BlizzardSoA::new(INITIAL_PARTICLES);

        for _ in 0..INITIAL_PARTICLES {
            soa.push(&mut rng);
        }

        Self {
            soa,
            fps_counter: FpsCounter::new(),
            time: 0.0,
            title_timer: 0.0,
            text_timer: 0.0,
            text_enabled: true,
            show_fps: true,
            burst_lines: Vec::new(),
            timeline_timer: 0.0,
        }
    }

    fn update_particles(&mut self, dt: f32) {
        // Pass 1: Physics and Transforms
        // We only zip exactly the arrays needed for physics.
        let physics_iter = self
            .soa
            .positions
            .iter_mut()
            .zip(self.soa.velocities.iter_mut())
            .zip(self.soa.angles.iter_mut())
            .zip(self.soa.spins.iter());

        for (((pos, vel), angle), spin) in physics_iter {
            vel.y += GRAVITY * dt;
            *vel *= 1.0 - dt * 0.1;

            *pos += *vel * dt;
            *angle += *spin * dt;

            if pos.x < -BOUNDS {
                vel.x = vel.x.abs();
                pos.x = -BOUNDS;
            } else if pos.x > BOUNDS {
                vel.x = -vel.x.abs();
                pos.x = BOUNDS;
            }
            if pos.y < -BOUNDS {
                vel.y = vel.y.abs();
                pos.y = -BOUNDS;
            } else if pos.y > BOUNDS {
                vel.y = -vel.y.abs();
                pos.y = BOUNDS;
            }
        }

        // Pass 2: Animation
        // Processed sequentially in its own tight loop.
        for anim in self.soa.anims.iter_mut() {
            anim.update(dt);
        }
    }

    fn render_particles(&self, engine: &mut Engine) {
        let tc = engine.get_texture_controller();

        let render_iter = self
            .soa
            .positions
            .iter()
            .zip(self.soa.sizes.iter())
            .zip(self.soa.angles.iter())
            .zip(self.soa.colors.iter())
            .zip(self.soa.anims.iter());

        for ((((pos, size), angle), color), anim) in render_iter {
            let uv = anim.current_uv();
            let color_vec = Vec4::new(color.r, color.g, color.b, color.a);
            let inst = SpriteInstance::new(*pos, Vec2::splat(*size), *angle, uv, color_vec);
            tc.add_instance("blizzard", inst);
        }
    }

    fn update_text_bursts(&mut self, dt: f32) {
        if self.text_enabled {
            self.text_timer += dt;
            if self.text_timer >= 0.5 {
                self.text_timer = 0.0;
                let mut rng = rand::rng();
                let messages = [
                    "Blizzard!",
                    "Data Oriented!",
                    "Stress test!",
                    "FPS?",
                    "SoA Fast!",
                    "Engine go brr",
                    "Cache hit!",
                    "More particles!",
                    "Render everything",
                ];
                let msg = messages[rng.random_range(0..messages.len())];
                let x = (rng.random::<f32>() - 0.5) * 800.0 + 400.0;
                let y = (rng.random::<f32>()) * 300.0 + 50.0;
                let burst = BurstLine {
                    effect: TypewriterEffect::new(
                        msg,
                        TextSpeed::Fast,
                        x,
                        y,
                        TextStyle::new(24.0).with_color(Color::WHITE),
                        PunctuationConfig::default(),
                    ),
                    lifetime: 0.0,
                };
                self.burst_lines.push(burst);
                if self.burst_lines.len() > 20 {
                    self.burst_lines.remove(0);
                }
            }
        }

        for b in &mut self.burst_lines {
            b.effect.update(dt);
            b.lifetime += dt;
        }
        self.burst_lines.retain(|b| b.lifetime < 3.0);
    }

    fn replay_timeline_if_due(&mut self, engine: &mut Engine) {
        self.timeline_timer += engine.delta_time();
        if self.timeline_timer >= 3.0 {
            self.timeline_timer = 0.0;
            let steps = TimelineBuilder::new()
                .parallel(vec![
                    (Animation::FadeIn { duration: 0.4 }, Easing::Linear),
                    (
                        Animation::SlideIn {
                            from: Direction::Top,
                            distance: 0.3,
                            duration: 0.4,
                        },
                        Easing::EaseOut,
                    ),
                ])
                .gap(0.1)
                .single(Animation::FadeOut { duration: 0.3 }, Easing::Linear)
                .build();
            engine.get_animation_system().start_timeline(steps);
        }
    }

    fn update_title_bar(&mut self, engine: &mut Engine) {
        if !self.show_fps {
            return;
        }
        self.title_timer += engine.delta_time();
        if self.title_timer >= 0.1 {
            self.title_timer = 0.0;
            engine.set_title(&format!(
                "Particle Blizzard SoA | {} particles | FPS: {:.0} | Frame: {:.1}ms | Min: {:.0} Max: {:.0}",
                self.soa.len(),
                self.fps_counter.fps(),
                self.fps_counter.frame_time_ms(),
                self.fps_counter.min_fps(),
                self.fps_counter.max_fps(),
            ));
        }
    }
}

impl Game for ParticleBlizzard {
    fn init(&mut self, engine: &mut Engine) {
        println!("╔══════════════════════════════════════════════════════╗");
        println!("║          PARTICLE BLIZZARD  –  SoA EDITION           ║");
        println!("╠══════════════════════════════════════════════════════╣");
        println!("║  Data-Oriented memory layout (Structure of Arrays)   ║");
        println!("╠══════════════════════════════════════════════════════╣");
        println!("║  UP    – +10 000 particles                           ║");
        println!("║  DOWN  – -10 000 particles                           ║");
        println!("║  SPACE – shake + text burst                          ║");
        println!("║  T     – toggle text spam                            ║");
        println!("║  F     – toggle FPS in title                         ║");
        println!("║  ESC   – quit                                        ║");
        println!("╚══════════════════════════════════════════════════════╝");

        if let Err(e) = engine.get_texture_controller().load_texture(
            include_bytes!("../src/static/textures/mistral.png"),
            "blizzard",
        ) {
            eprintln!("Texture load error: {e}");
        }

        engine.get_camera().set_zoom(1.0);
        println!("✓ Ready – {} particles spawned", self.soa.len());
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps_counter.update(dt);
        self.time += dt;

        self.update_particles(dt);
        self.render_particles(engine);
        self.update_text_bursts(dt);
        self.replay_timeline_if_due(engine);
        self.update_title_bar(engine);
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Escape)
        {
            println!("\n=== Final Stats ===");
            println!("Particles: {}", self.soa.len());
            println!("Avg FPS:   {:.1}", self.fps_counter.fps());
            println!("Min FPS:   {:.1}", self.fps_counter.min_fps());
            println!("Max FPS:   {:.1}", self.fps_counter.max_fps());
            std::process::exit(0);
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::ArrowUp)
        {
            let mut rng = rand::rng();
            for _ in 0..10_000 {
                self.soa.push(&mut rng);
            }
            println!("Particles: {}", self.soa.len());
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::ArrowDown)
        {
            let new_len = self.soa.len().saturating_sub(10_000);
            self.soa.truncate(new_len);
            println!("Particles: {}", self.soa.len());
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Space)
        {
            engine.get_camera().add_trauma(1.0);
            self.text_timer = 0.5;
        }

        if engine.get_event_queue().was_key_just_pressed(KeyCode::KeyT) {
            self.text_enabled = !self.text_enabled;
            println!(
                "Text spam: {}",
                if self.text_enabled { "ON" } else { "OFF" }
            );
        }

        if engine.get_event_queue().was_key_just_pressed(KeyCode::KeyF) {
            self.show_fps = !self.show_fps;
        }
    }
}

fn main() {
    let config = WindowConfig {
        title: "Stress: Particle Blizzard SoA".to_string(),
        width: 1920,
        height: 1080,
        resizable: false,
        fullscreen: false,
        vsync: false,
        background_color: Color::new(0.02, 0.04, 0.08, 1.0),
        language: Language::resolve("en_us").unwrap(),
    };
    app::run(config, Box::new(ParticleBlizzard::new())).expect("Particle blizzard failed");
}
