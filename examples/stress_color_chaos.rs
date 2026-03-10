//! **Color Chaos Stress Test** — Renders tens of thousands of sprites, each
//! with a unique color that lerps through the spectrum every frame.
//!
//! Engine systems under stress:
//!   • `Color::lerp` + `Color::with_alpha` every frame per sprite
//!   • `SpriteInstance` bulk submission
//!   • Camera shake trauma decay
//!   • Rolling FPS counter
//!   • Event queue key-state tracking
//!
//! Controls:
//!   UP    – Add 5 000 sprites
//!   DOWN  – Remove 5 000 sprites
//!   SPACE – Trigger full-screen camera shake
//!   C     – Randomize base color palette
//!   ESC   – Quit

use glam::{Vec2, Vec4};
use rand::RngExt;
use rustgames::core::app;
use rustgames::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const INITIAL_COUNT: usize = 20_000;
const BOUNDS_X: f32 = 640.0;
const BOUNDS_Y: f32 = 360.0;
const SPRITE_SIZE: f32 = 8.0;

// ---------------------------------------------------------------------------
// Per-sprite state
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct ColorSprite {
    pos: Vec2,
    vel: Vec2,
    hue_offset: f32, // [0..1] phase offset for color cycling
    alpha_phase: f32,
}

// ---------------------------------------------------------------------------
// Game state
// ---------------------------------------------------------------------------

struct ColorChaos {
    sprites: Vec<ColorSprite>,
    fps_counter: FpsCounter,
    time: f32,
    title_timer: f32,
    palette: [Color; 4],
}

impl ColorChaos {
    fn new() -> Self {
        let mut rng = rand::rng();
        let mut sprites = Vec::with_capacity(INITIAL_COUNT);
        for _ in 0..INITIAL_COUNT {
            sprites.push(Self::random_sprite(&mut rng));
        }
        Self {
            sprites,
            fps_counter: FpsCounter::new(),
            time: 0.0,
            title_timer: 0.0,
            palette: [Color::RED, Color::CYAN, Color::YELLOW, Color::MAGENTA],
        }
    }

    fn random_sprite(rng: &mut impl rand::Rng) -> ColorSprite {
        ColorSprite {
            pos: Vec2::new(
                (rng.random::<f32>() - 0.5) * BOUNDS_X * 2.0,
                (rng.random::<f32>() - 0.5) * BOUNDS_Y * 2.0,
            ),
            vel: Vec2::new(
                (rng.random::<f32>() - 0.5) * 120.0,
                (rng.random::<f32>() - 0.5) * 120.0,
            ),
            hue_offset: rng.random::<f32>(),
            alpha_phase: rng.random::<f32>() * std::f32::consts::TAU,
        }
    }

    fn randomize_palette(&mut self) {
        let mut rng = rand::rng();
        for slot in &mut self.palette {
            *slot = Color::new(
                rng.random::<f32>(),
                rng.random::<f32>(),
                rng.random::<f32>(),
                1.0,
            );
        }
    }

    fn compute_color(&self, sprite: &ColorSprite) -> Vec4 {
        // Cycle through all 4 palette colors
        let t = ((self.time * 0.5 + sprite.hue_offset) % 1.0 * 4.0).fract();
        let idx = ((self.time * 0.5 + sprite.hue_offset) * 4.0) as usize % 4;
        let next_idx = (idx + 1) % 4;
        let color = self.palette[idx].lerp(self.palette[next_idx], t);

        // Pulsing alpha: 0.3..1.0
        let alpha = 0.3 + 0.35 * (sprite.alpha_phase + self.time * 3.0).sin().abs();
        let color = color.with_alpha(alpha);
        Vec4::new(color.r, color.g, color.b, color.a)
    }

    fn update_physics(&mut self, dt: f32) {
        for s in &mut self.sprites {
            s.pos += s.vel * dt;

            if s.pos.x.abs() > BOUNDS_X {
                s.vel.x = -s.vel.x;
                s.pos.x = s.pos.x.clamp(-BOUNDS_X, BOUNDS_X);
            }
            if s.pos.y.abs() > BOUNDS_Y {
                s.vel.y = -s.vel.y;
                s.pos.y = s.pos.y.clamp(-BOUNDS_Y, BOUNDS_Y);
            }

            s.alpha_phase += dt * 2.0;
        }
    }

    fn render(&self, engine: &mut Engine) {
        let tc = engine.get_texture_controller();
        for sprite in &self.sprites {
            let color = self.compute_color(sprite);
            let inst = SpriteInstance::new(
                sprite.pos,
                Vec2::splat(SPRITE_SIZE),
                sprite.hue_offset * std::f32::consts::TAU,
                Vec4::new(0.0, 0.0, 1.0, 1.0), // full UV
                color,
            );
            tc.add_instance("sprite", inst);
        }
    }

    fn update_title(&mut self, engine: &mut Engine) {
        self.title_timer += engine.delta_time();
        if self.title_timer >= 0.05 {
            self.title_timer = 0.0;
            engine.set_title(&format!(
                "Color Chaos | Sprites: {} | FPS: {:.0} | Frame: {:.1}ms | Min: {:.0} Max: {:.0}",
                self.sprites.len(),
                self.fps_counter.fps(),
                self.fps_counter.frame_time_ms(),
                self.fps_counter.min_fps(),
                self.fps_counter.max_fps(),
            ));
        }
    }
}

impl Game for ColorChaos {
    fn init(&mut self, engine: &mut Engine) {
        println!("=== Color Chaos Stress Test ===");
        println!("Sprites: {}", self.sprites.len());
        println!();
        println!("Controls:");
        println!("  UP    - Add 5 000 sprites");
        println!("  DOWN  - Remove 5 000 sprites");
        println!("  SPACE - Camera shake");
        println!("  C     - Randomize palette");
        println!("  ESC   - Exit");

        if let Err(e) = engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/mistral.png"), "sprite")
        {
            eprintln!("Texture load error: {e}");
        }

        engine.get_camera().set_zoom(1.0);
        println!("✓ Initialized");
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps_counter.update(dt);
        self.time += dt;
        self.update_physics(dt);
        self.render(engine);
        self.update_title(engine);
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        if engine.get_event_queue().was_key_just_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        if engine.get_event_queue().was_key_just_pressed(KeyCode::Space) {
            engine.get_camera().add_trauma(0.9);
        }

        if engine.get_event_queue().was_key_just_pressed(KeyCode::ArrowUp) {
            let mut rng = rand::rng();
            for _ in 0..5_000 {
                self.sprites.push(Self::random_sprite(&mut rng));
            }
            println!("Sprites: {}", self.sprites.len());
        }

        if engine.get_event_queue().was_key_just_pressed(KeyCode::ArrowDown) {
            let new_len = self.sprites.len().saturating_sub(5_000);
            self.sprites.truncate(new_len);
            println!("Sprites: {}", self.sprites.len());
        }

        if engine.get_event_queue().was_key_just_pressed(KeyCode::KeyC) {
            self.randomize_palette();
            println!("Palette randomized");
        }
    }
}

fn main() {
    let config = WindowConfig {
        title: "Stress: Color Chaos".to_string(),
        width: 1280,
        height: 720,
        resizable: false,
        fullscreen: false,
        vsync: false, // vsync off for max stress
        background_color: Color::BLACK,
        language: Language::resolve("en_us").unwrap(),
    };
    app::run(config, Box::new(ColorChaos::new())).expect("Color chaos failed");
}

