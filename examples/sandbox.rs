use rustgames::core::{app, Engine};
use rustgames::text::{TextSpeed, TextStyle};
use rustgames::core::game::Game as CoreGame;

pub struct Game {
    score: f32,
    score_text_id: usize,
}

impl CoreGame for Game {
    fn init(&mut self, engine: &mut Engine) {
        let style = TextStyle::new(64.0);
        engine.get_text_controller().set_style(style);

        engine.get_typewriter_controller().add_effect("Luna", TextSpeed::Fast, 100.0, 850.0);

        engine.get_typewriter_controller().add_effect("Привет, я Луна! Как твои дела?", TextSpeed::Slow, 100.0, 800.0);
        self.score_text_id = engine.get_typewriter_controller().add_effect(&format!("[b]Очки[/b]: {}", self.score), TextSpeed::Instant, 50.0, 50.0);

        engine.load_texture("mistral", include_bytes!("../src/mistral.png"));
        engine.load_texture("background", include_bytes!("../src/OIP-475081084.jpg"));
        engine.get_camera_controller().shake(10.0, 1.0);
    }

    fn update(&mut self, engine: &mut Engine) {
        self.score += 0.1;
        let text = format!("[b]Очки[/b]: {}", self.score as u32);

        engine.get_typewriter_controller().effect_mut(self.score_text_id).unwrap().set_text(&text, TextSpeed::Instant);
    }
}

fn main() {
    println!("Запускаем движок...");


    let game = Game {
        score: 0.0,
        score_text_id: 0,
    };

    let _app = match app::run("RenPy Style Game", 1280.0, 720.0, Box::new(game)) {
        Ok(app) => app,
        Err(e) => panic!("{}", e),
    };
}