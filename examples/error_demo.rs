//! # error_demo — example of engine error diagnostics
//!
//! This example intentionally triggers various engine errors to demonstrate the error handling and diagnostics system in RsGames. Each step is designed to fail in a different way, showcasing how the engine reports errors without panicking.
//!
//! Run: cargo run --example error_demo

use rustgames::core::app;
use rustgames::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────

struct ErrorDemo {
    step: u32,
    timer: f32,
}

impl Game for ErrorDemo {
    fn init(&mut self, engine: &mut Engine) {
        println!();
        println!("╔══════════════════════════════════════════════════════════╗");
        println!("║         RsGames  —  Error Diagnostics Demo               ║");
        println!("║  Each step triggers a real engine error on purpose.      ║");
        println!("╚══════════════════════════════════════════════════════════╝");
        println!();

        println!("── [Step 1] Loading a sound from a path that does not exist…");
        if let Err(e) = engine
            .get_audio_system()
            .load_sound("ghost", "/tmp/definitely_not_here.mp3")
        {
            eprintln!("{e}");
        }

        println!("── [Step 2] Loading a texture from garbage bytes…");
        let garbage: &[u8] = b"this is not an image at all!!!";
        if let Err(e) = engine
            .get_texture_controller()
            .load_texture(garbage, "broken_texture")
        {
            // Code G006
            eprintln!("{e}");
        }

        println!("── [Step 3] Playing a sound that was never loaded…");
        if let Err(e) = engine.get_audio_system().play("missing_sound") {
            // Code A002
            eprintln!("{e}");
        }

        println!("── [Step 4] Loading a font from a path that does not exist…");
        if let Err(e) = Font::load("/tmp/no_such_font.ttf") {
            use rustgames::error::TextError;
            let diag = TextError::FontLoadFailed("/tmp/no_such_font.ttf".to_string(), e);
            eprintln!("{diag}");
        }

        println!("── [Step 5] Creating a font from invalid bytes…");
        if let Err(e) = Font::from_bytes("bad_font", b"not a ttf file at all".to_vec()) {
            // Code T002
            eprintln!("{e}");
        }

        println!("── [Step 6] Loading sounds from a directory that doesn't exist…");
        if let Err(e) = engine
            .get_audio_system()
            .load_sound_dir("/tmp/no_audio_dir_here")
        {
            // Code A004
            eprintln!("{e}");
        }

        println!("── [Step 7] Loading textures from a directory that doesn't exist…");
        engine
            .get_texture_controller()
            .load_texture_dir("/tmp/no_textures_here");

        println!();
        println!("All 7 intentional errors were displayed above.");
        println!("Press ESC to exit.");
        println!();
    }

    fn update(&mut self, _engine: &mut Engine) {}

    fn handle_update(&mut self, engine: &mut Engine) {
        self.timer += engine.delta_time();

        if self.step == 0 && self.timer > 2.0 {
            self.step = 1;
            println!("── [Step 8 / deferred] Playing another missing sound after 2s…");
            if let Err(e) = engine.get_audio_system().play("another_ghost") {
                eprintln!("{e}");
            }
        }

        if engine
            .get_event_queue()
            .was_key_just_pressed(KeyCode::Escape)
        {
            std::process::exit(0);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    let game = ErrorDemo { step: 0, timer: 0.0 };

    let window_config = WindowConfig {
        title: "RsGames — Error Diagnostics Demo".to_string(),
        width: 800,
        height: 200,
        resizable: false,
        fullscreen: false,
        vsync: true,
        background_color: Color::BLACK,
        language: Language::resolve("en_us")
            .unwrap_or_else(|| Language::new("en_us".to_string(), "English".to_string())),
    };

    app::run(window_config, Box::new(game)).expect("Failed to run error demo");
}

