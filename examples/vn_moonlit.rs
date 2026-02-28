//! **Moonlit Confession** — A short 2D Visual Novel demo.
//!
//! A romantic story under the moonlight with branching dialogue.
//! Built entirely with the `rustgames` engine.
//!
//! Controls: SPACE — advance dialogue, 1/2 — choose option, ESC — quit

use glam::Vec2;
use rustgames::core::app;
use rustgames::graphics::VfxRenderer;
use rustgames::prelude::*;

const W: f32 = 1280.0;
const H: f32 = 720.0;

#[derive(Clone)]
struct DialogueLine {
    speaker: &'static str,
    text: &'static str,
}

struct Choice {
    prompt: &'static str,
    option_a: &'static str,
    option_b: &'static str,
    next_a: usize,
    next_b: usize,
}

struct MoonlitConfession {
    lines: Vec<DialogueLine>,
    choices: Vec<Choice>,
    current: usize,
    tw_id: usize,
    vfx: VfxRenderer,
    fps: FpsCounter,
    waiting_choice: Option<usize>,
    chose_kind: bool,
    title_timer: f32,
    ended: bool,
}

impl MoonlitConfession {
    fn build_script() -> Vec<DialogueLine> {
        vec![
            dl(
                "???",
                "[color=FF0000]The moonlight[/color] paints the old bridge silver...",
            ),
            dl("Hana", "I didn't think you'd actually come."),
            dl("You", "I almost didn't. The rain nearly stopped me."),
            dl("Hana", "But it stopped, didn't it? Almost like fate."),
            dl("You", "(She smiles. My heart races.)"),
            dl("Hana", "I have something I need to tell you..."),
            dl("Hana", "I'm leaving the city next week."),
            dl("You", "What?! Why didn't you say sooner?"),
            dl("Hana", "I was scared. Scared of your answer."),
            dl("You", "Then let me walk with you. Wherever you go."),
            dl("Hana", "You... you mean that?"),
            dl("You", "Every word."),
            dl("Hana", "Thank you. That means everything."),
            dl("You", "Then we'd better make tonight count."),
            dl("Hana", "Ha! Same reckless spirit as always."),
            dl("You", "That's why you like me, right?"),
            dl("Hana", "...Maybe."),
            dl("???", "They stood on the bridge until dawn."),
            dl("???", "THE END — Press ESC"),
        ]
    }

    fn build_choices() -> Vec<Choice> {
        vec![Choice {
            prompt: "What do you say?",
            option_a: "1: Be kind and supportive",
            option_b: "2: Be bold and adventurous",
            next_a: 10,
            next_b: 14,
        }]
    }
}

fn dl(speaker: &'static str, text: &'static str) -> DialogueLine {
    DialogueLine { speaker, text }
}

impl Game for MoonlitConfession {
    fn init(&mut self, engine: &mut Engine) {
        engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/sakura.png"), "bg");
        engine.get_camera().set_zoom(1.0);
        self.show_line(engine, 0);
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps.update(dt);
        self.vfx.update(dt);
        self.title_timer += dt;
        if self.title_timer > 0.5 {
            self.title_timer = 0.0;
            engine.set_title(&format!("Moonlit Confession | FPS: {:.0}", self.fps.fps()));
        }
        self.draw(engine);
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        self.handle_input(engine);
    }
}

impl MoonlitConfession {
    fn draw(&mut self, engine: &mut Engine) {
        engine.get_texture_controller().use_texture(
            "bg",
            Vec2::new(W * 2.0, H * 2.0),
            Vec2::ZERO,
            0.0,
            0.6,
        );
    }

    fn show_line(&mut self, engine: &mut Engine, idx: usize) {
        self.current = idx;
        if idx >= self.lines.len() {
            self.ended = true;
            return;
        }
        let line = &self.lines[idx];
        let full = format!("[{}]: {}", line.speaker, line.text);
        engine.get_text_system().remove_text(self.tw_id);
        self.tw_id = engine.get_text_system().add_text(
            full,
            TextSpeed::Fast,
            60.0,
            H - 120.0,
            TextStyle::new(64.0),
            PunctuationConfig::default(),
        );
    }

    fn handle_input(&mut self, engine: &mut Engine) {
        let space;
        let key1;
        let key2;
        let esc;
        {
            let eq = engine.get_event_queue();
            space = eq.was_key_just_pressed(KeyCode::Space);
            key1 = eq.was_key_just_pressed(KeyCode::Digit1);
            key2 = eq.was_key_just_pressed(KeyCode::Digit2);
            esc = eq.was_key_just_pressed(KeyCode::Escape);
        }
        if esc {
            std::process::exit(0);
        }
        if self.ended {
            return;
        }
        if let Some(ci) = self.waiting_choice {
            self.handle_choice(engine, ci, key1, key2);
        } else if space {
            self.advance(engine);
        }
    }

    fn handle_choice(&mut self, engine: &mut Engine, ci: usize, k1: bool, k2: bool) {
        let c = &self.choices[ci];
        if k1 {
            self.chose_kind = true;
            self.waiting_choice = None;
            self.show_line(engine, c.next_a);
        } else if k2 {
            self.chose_kind = false;
            self.waiting_choice = None;
            self.show_line(engine, c.next_b);
        }
    }

    fn advance(&mut self, engine: &mut Engine) {
        if !engine.get_text_system().is_complete(self.tw_id) {
            engine.get_text_system().skip(self.tw_id);
            return;
        }
        let next = self.current + 1;
        if next == 9 {
            self.waiting_choice = Some(0);
            let c = &self.choices[0];
            let txt = format!("{}\n{}\n{}", c.prompt, c.option_a, c.option_b);
            engine.get_text_system().remove_text(self.tw_id);
            self.tw_id = engine.get_text_system().add_text(
                txt,
                TextSpeed::Instant,
                60.0,
                H - 120.0,
                TextStyle::new(64.0),
                PunctuationConfig::default(),
            );
            return;
        }
        let next = if self.current == 13 || self.current == 17 {
            18
        } else {
            next
        };
        self.show_line(engine, next);
    }
}

fn main() {
    let game = MoonlitConfession {
        lines: MoonlitConfession::build_script(),
        choices: MoonlitConfession::build_choices(),
        current: 0,
        tw_id: 0,
        vfx: VfxRenderer::new(),
        fps: FpsCounter::new(),
        waiting_choice: None,
        chose_kind: false,
        title_timer: 0.0,
        ended: false,
    };
    let config = WindowConfig {
        title: "Moonlit Confession".into(),
        width: W as u32,
        height: H as u32,
        resizable: true,
        fullscreen: true,
        vsync: true,
        background_color: Color::new(0.05, 0.0, 0.1, 1.0),
    };
    app::run(config, Box::new(game)).expect("Failed to run");
}
