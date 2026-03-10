//! **The Last Train** — A mystery Visual Novel demo.
//!
//! You wake up alone on a moving train with no memory.
//! Built entirely with the `rustgames` engine.
//!
//! Controls: SPACE — advance, 1/2 — choose, ESC — quit

use glam::Vec2;
use rustgames::core::app;
use rustgames::graphics::VfxRenderer;
use rustgames::prelude::*;

const W: f32 = 1280.0;
const H: f32 = 720.0;

struct Line {
    speaker: &'static str,
    text: &'static str,
}

fn l(s: &'static str, t: &'static str) -> Line {
    Line {
        speaker: s,
        text: t,
    }
}

struct LastTrain {
    script: Vec<Line>,
    idx: usize,
    tw_id: usize,
    vfx: VfxRenderer,
    fps: FpsCounter,
    title_t: f32,
    ended: bool,
    choosing: bool,
    trust_conductor: bool,
}

impl LastTrain {
    fn script() -> Vec<Line> {
        vec![
            l("???", "The rhythmic clatter of wheels on tracks..."),
            l("You", "Where... where am I?"),
            l("You", "A train. Empty seats. Night outside the window."),
            l("You", "My head is pounding. I can't remember anything."),
            l("???", "A door slides open behind you."),
            l("Conductor", "Ah, you're awake. Ticket, please."),
            l("You", "I don't... I don't have a ticket."),
            l("Conductor", "Everyone on this train has a ticket."),
            l("Conductor", "Check your pocket."),
            l("You", "I reach into my coat. A crumpled paper."),
            l("You", "It reads: 'ONE WAY — DESTINATION: ???'"),
            l("Conductor", "See? You belong here, just like everyone."),
            l("You", "Where is this train going?"),
            l("Conductor", "Where do you THINK it's going?"),
            l("You", "I trust you. Take me where I need to go."),
            l("Conductor", "A wise choice. Rest now."),
            l("You", "My eyes grow heavy. The train rocks gently."),
            l("???", "When you wake, sunlight floods the carriage."),
            l("You", "I don't trust you. Let me off this train."),
            l("Conductor", "There are no stops until the end."),
            l("You", "Then I'll find my own way."),
            l("???", "You pull the emergency brake. Sparks fly."),
            l("???", "The journey continues... or does it?"),
            l("???", "THE END — Press ESC"),
        ]
    }
}

impl Game for LastTrain {
    fn init(&mut self, engine: &mut Engine) {
        if let Err(e) = engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/space.png"), "bg")
        {
            eprintln!("{e}");
        }
        engine.get_camera().set_zoom(1.0);
        self.show(engine, 0);
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps.update(dt);
        self.vfx.update(dt);
        self.title_t += dt;
        if self.title_t > 0.5 {
            self.title_t = 0.0;
            engine.set_title(&format!("The Last Train | FPS: {:.0}", self.fps.fps()));
        }
        engine.get_texture_controller().use_texture(
            "bg",
            Vec2::new(W * 2.0, H * 2.0),
            Vec2::ZERO,
            0.0,
            0.5,
        );
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        let (space, k1, k2, esc) = read_vn_inputs(engine);
        if esc {
            std::process::exit(0);
        }
        if self.ended {
            return;
        }
        if self.choosing {
            if k1 {
                self.trust_conductor = true;
                self.choosing = false;
                self.show(engine, 15);
            } else if k2 {
                self.trust_conductor = false;
                self.choosing = false;
                self.show(engine, 19);
            }
        } else if space {
            self.advance(engine);
        }
    }
}

impl LastTrain {
    fn show(&mut self, engine: &mut Engine, i: usize) {
        self.idx = i;
        if i >= self.script.len() {
            self.ended = true;
            return;
        }
        let line = &self.script[i];
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

    fn advance(&mut self, engine: &mut Engine) {
        if !engine.get_text_system().is_complete(self.tw_id) {
            engine.get_text_system().skip(self.tw_id);
            return;
        }
        let next = self.idx + 1;
        if next == 14 {
            self.choosing = true;
            let txt = "What do you do?\n\
                       1: Trust the conductor\n\
                       2: Demand to leave";
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
        let next = if self.idx == 18 || self.idx == 22 {
            23
        } else {
            next
        };
        self.show(engine, next);
    }
}

fn read_vn_inputs(engine: &mut Engine) -> (bool, bool, bool, bool) {
    let eq = engine.get_event_queue();
    (
        eq.was_key_just_pressed(KeyCode::Space),
        eq.was_key_just_pressed(KeyCode::Digit1),
        eq.was_key_just_pressed(KeyCode::Digit2),
        eq.was_key_just_pressed(KeyCode::Escape),
    )
}

fn main() {
    let game = LastTrain {
        script: LastTrain::script(),
        idx: 0,
        tw_id: 0,
        vfx: VfxRenderer::new(),
        fps: FpsCounter::new(),
        title_t: 0.0,
        ended: false,
        choosing: false,
        trust_conductor: false,
    };
    let config = WindowConfig {
        title: "The Last Train".into(),
        width: W as u32,
        height: H as u32,
        resizable: true,
        fullscreen: true,
        vsync: true,
        background_color: Color::new(0.02, 0.02, 0.05, 1.0),
        language: Language::resolve("en_us").expect("Failed to resolve language"),
    };
    app::run(config, Box::new(game)).expect("Failed to run");
}
