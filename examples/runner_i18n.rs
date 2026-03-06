//! **Runner i18n** — Simple runner game with movement and i18n translation demo.
//!
//! A player square moves around the screen. All UI labels are translated
//! via the `rustgames` translation system (English / Russian).
//!
//! Controls:
//!   Arrow keys / WASD — Move player
//!   L                 — Toggle language (EN ↔ RU)
//!   ESC               — Quit

use glam::Vec2;
use rustgames::core::app;
use rustgames::prelude::*;
use rustgames::text::TextData;
use rustgames::translation::{generate_id_from_name, DictionarySystem, Translation, TranslationSystem};

// ── screen size ──────────────────────────────────────────────────────────────
const W: f32 = 2560.0;
const H: f32 = 1440.0;

// ── movement ─────────────────────────────────────────────────────────────────
const PLAYER_SPEED: f32 = 300.0;
const PLAYER_SIZE: f32 = 64.0;

// ── translation keys (stable IDs derived from the English key string) ────────
const KEY_TITLE: &str = "ui.title";
const KEY_SCORE: &str = "ui.score";
const KEY_LANG: &str = "ui.language";
const KEY_HINT: &str = "ui.hint";

// ── languages ────────────────────────────────────────────────────────────────
const LANG_EN: &str = "en_us";
const LANG_RU: &str = "ru_ru";

// ─────────────────────────────────────────────────────────────────────────────
// Game state
// ─────────────────────────────────────────────────────────────────────────────
struct RunnerI18n {
    player_pos: Vec2,
    score: u32,
    fps: FpsCounter,
    title_timer: f32,
    /// Currently selected language small_name
    current_lang: &'static str,
    /// Typewriter ids for the HUD labels
    tw_title: usize,
    tw_score: usize,
    tw_lang: usize,
    tw_hint: usize,
    /// Whether the labels have been created (skip first frame re-creation)
    hud_ready: bool,
}

impl RunnerI18n {
    fn new() -> Self {
        Self {
            player_pos: Vec2::new(W / 2.0, H / 2.0),
            score: 0,
            fps: FpsCounter::new(),
            title_timer: 0.0,
            current_lang: LANG_EN,
            tw_title: 0,
            tw_score: 0,
            tw_lang: 0,
            tw_hint: 0,
            hud_ready: false,
        }
    }

    // ── translation setup ────────────────────────────────────────────────────

    /// Build dictionary (fallback texts in English) and translations for each language.
    fn build_translations() -> (DictionarySystem, TranslationSystem) {
        let mut dict = DictionarySystem::new();
        let mut trans = TranslationSystem::new();

        // Register fallback dictionary entries (English text as fallback)
        let entries: &[(&str, &str)] = &[
            (KEY_TITLE, "Runner i18n Demo"),
            (KEY_SCORE, "Score: {0}"),
            (KEY_LANG, "Language: English"),
            (KEY_HINT, "WASD/Arrows=Move  L=Lang  ESC=Quit"),
        ];
        for (key, fallback) in entries {
            dict.add_dictionary(key);
            // Also add the English text as a real dictionary entry so the
            // fallback shows something useful before any language is set.
            dict.add_dictionary_entry(generate_id_from_name(key), fallback);
        }

        // English translations
        let en_id = generate_id_from_name(LANG_EN);
        let ru_id = generate_id_from_name(LANG_RU);

        let en_translations: &[(&str, &str)] = &[
            (KEY_TITLE, "Runner i18n Demo"),
            (KEY_SCORE, "Score: {0}"),
            (KEY_LANG, "Language: English"),
            (KEY_HINT, "WASD/Arrows=Move  L=Lang  ESC=Quit"),
        ];

        let ru_translations: &[(&str, &str)] = &[
            (KEY_TITLE, "Демо: i18n-бегун"),
            (KEY_SCORE, "Счёт: {0}"),
            (KEY_LANG, "Язык: Русский"),
            (KEY_HINT, "WASD/Стрелки=Движение  L=Язык  ESC=Выход"),
        ];

        for (key, text) in en_translations {
            let tid = generate_id_from_name(key);
            trans.add_translation(Translation::new(tid, en_id, text.to_string()));
        }
        for (key, text) in ru_translations {
            let tid = generate_id_from_name(key);
            trans.add_translation(Translation::new(tid, ru_id, text.to_string()));
        }

        (dict, trans)
    }

    // ── HUD creation/refresh ─────────────────────────────────────────────────

    fn create_hud(&mut self, engine: &mut Engine) {
        let ts = engine.get_text_system();
        // Remove old effects if they exist
        if self.hud_ready {
            ts.remove_text(self.tw_title);
            ts.remove_text(self.tw_score);
            ts.remove_text(self.tw_lang);
            ts.remove_text(self.tw_hint);
        }

        let style_large = TextStyle::new(48.0);
        let style_normal = TextStyle::new(32.0);
        let style_small = TextStyle::new(28.0);
        let punc = PunctuationConfig::default();

        // Title — resolved via translation system
        self.tw_title = ts.add_text_by_id(TextData {
            text: "Runner i18n Demo".to_string(),
            text_id: generate_id_from_name(KEY_TITLE),
            speed: TextSpeed::Instant,
            x: 40.0,
            y: 20.0,
            style: style_large,
            punctuation_config: punc,
        });

        // Score label
        let score_fallback = format!("Score: {}", self.score);
        self.tw_score = ts.add_text_by_id(TextData {
            text: score_fallback,
            text_id: generate_id_from_name(KEY_SCORE),
            speed: TextSpeed::Instant,
            x: 40.0,
            y: 80.0,
            style: style_normal.clone(),
            punctuation_config: punc,
        });

        // Language label
        self.tw_lang = ts.add_text_by_id(TextData {
            text: "Language: English".to_string(),
            text_id: generate_id_from_name(KEY_LANG),
            speed: TextSpeed::Instant,
            x: 40.0,
            y: 120.0,
            style: style_normal,
            punctuation_config: punc,
        });

        // Hint at bottom
        self.tw_hint = ts.add_text_by_id(TextData {
            text: "WASD/Arrows=Move  L=Lang  ESC=Quit".to_string(),
            text_id: generate_id_from_name(KEY_HINT),
            speed: TextSpeed::Instant,
            x: 40.0,
            y: H - 50.0,
            style: style_small,
            punctuation_config: punc,
        });

        self.hud_ready = true;
    }

    // ── movement ─────────────────────────────────────────────────────────────

    fn handle_movement(&mut self, engine: &mut Engine, dt: f32) {
        let mut vel = Vec2::ZERO;

        self.keyboard_input(engine, &mut vel);

        if vel != Vec2::ZERO {
            self.player_pos += vel.normalize() * PLAYER_SPEED * dt;
            self.score += 1;
        }

        let margin = Vec2::splat(PLAYER_SIZE / 2.0);
        self.player_pos = self.player_pos.clamp(margin, Vec2::new(W, H) - margin);
    }

    fn keyboard_input(&mut self, engine: &mut Engine, vel: &mut Vec2) {
        let eq = engine.get_event_queue();
        if eq.is_key_down(KeyCode::ArrowLeft) || eq.is_key_down(KeyCode::KeyA) { vel.x -= 1.0; }
        if eq.is_key_down(KeyCode::ArrowRight) || eq.is_key_down(KeyCode::KeyD) { vel.x += 1.0; }
        if eq.is_key_down(KeyCode::ArrowUp) || eq.is_key_down(KeyCode::KeyW) { vel.y += 1.0; }
        if eq.is_key_down(KeyCode::ArrowDown) || eq.is_key_down(KeyCode::KeyS) { vel.y -= 1.0; }
    }

    // ── language toggle ──────────────────────────────────────────────────────

    fn toggle_language(&mut self, engine: &mut Engine) {
        self.current_lang = if self.current_lang == LANG_EN {
            LANG_RU
        } else {
            LANG_EN
        };
        engine.set_language(self.current_lang);
        self.create_hud(engine);
    }

    // ── rendering ────────────────────────────────────────────────────────────

    fn draw_player(&self, engine: &mut Engine) {
        engine.get_texture_controller().use_texture(
            "player",
            Vec2::splat(PLAYER_SIZE),
            self.player_pos - Vec2::splat(PLAYER_SIZE / 2.0),
            0.0,
            1.0,
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Game trait
// ─────────────────────────────────────────────────────────────────────────────

impl Game for RunnerI18n {
    fn init(&mut self, engine: &mut Engine) {
        // Load textures
        if let Err(e) = engine.get_texture_controller().load_texture(
            include_bytes!("../src/static/textures/mistral.png"),
            "player",
        ) { eprintln!("{e}"); }
        if let Err(e) = engine.get_texture_controller().load_texture(
            include_bytes!("../src/static/textures/OIP-475081084.jpg"),
            "bg",
        ) { eprintln!("{e}"); }

        // Register languages
        engine.add_language(Language::new(LANG_EN.to_string(), "English".to_string()));
        engine.add_language(Language::new(LANG_RU.to_string(), "Русский".to_string()));

        // Register translations
        let (dict, trans) = RunnerI18n::build_translations();
        engine.save_translations(dict, trans);

        // Set initial language
        engine.set_language(LANG_EN);

        // Build initial HUD
        self.create_hud(engine);

        engine.get_camera().set_zoom(1.0);
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps.update(dt);
        self.title_timer += dt;

        // Draw background
        engine
            .get_texture_controller()
            .use_texture("bg", Vec2::new(W, H), Vec2::ZERO, 0.0, 0.3);

        // Draw player sprite
        self.draw_player(engine);

        // Update window title periodically
        if self.title_timer >= 0.5 {
            self.title_timer = 0.0;
            engine.set_title(&format!(
                "Runner i18n | Score: {} | Lang: {} | FPS: {:.0}",
                self.score,
                self.current_lang,
                self.fps.fps()
            ));
        }
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();

        self.handle_movement(engine, dt);

        let (toggle_lang, quit) = {
            let eq = engine.get_event_queue();
            (
                eq.was_key_just_pressed(KeyCode::KeyL),
                eq.was_key_just_pressed(KeyCode::Escape)
            )
        };

        if toggle_lang {
            self.toggle_language(engine);
        }
        if quit {
            std::process::exit(0);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    let game = RunnerI18n::new();

    let config = WindowConfig {
        title: "Runner i18n Demo".into(),
        width: W as u32,
        height: H as u32,
        resizable: true,
        fullscreen: true,
        vsync: true,
        background_color: Color::new(0.1, 0.1, 0.2, 1.0),
        language: Language::resolve("en_us").unwrap(),
    };

    app::run(config, Box::new(game)).expect("Failed to run runner_i18n");
}