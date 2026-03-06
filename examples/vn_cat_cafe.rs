//! **Cat Cafe Chronicles** — A cozy Visual Novel demo.
//!
//! Manage a magical cat cafe and befriend talking cats.
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

struct CatCafe {
    script: Vec<Line>,
    idx: usize,
    tw_id: usize,
    vfx: VfxRenderer,
    fps: FpsCounter,
    title_t: f32,
    ended: bool,
    choosing: bool,
    chose_mochi: bool,
    reputation: i32,
}

impl CatCafe {
    fn script() -> Vec<Line> {
        vec![
            l("???", "A tiny bell chimes as you push open the door."),
            l("You", "So this is the place. 'Paws & Whiskers Cafe'."),
            l("You", "Grandma left it to me in her will."),
            l("You", "Dusty tables, faded curtains... but cozy."),
            l("???", "Meow."),
            l("You", "A orange tabby sits on the counter, watching."),
            l("Mochi", "About time you showed up, human."),
            l("You", "Did that cat just... TALK?!"),
            l("Mochi", "Don't be dramatic. All cafe cats talk."),
            l("Mochi", "Your grandma knew. She was one of us."),
            l("You", "One of... what?"),
            l("Mochi", "A Cat Whisperer. And now, so are you."),
            l("Mochi", "But first, this place is a mess."),
            l("Mochi", "We need customers. And fish. Mostly fish."),
            l("You", "Alright Mochi, teach me the ways of the cafe."),
            l("Mochi", "First rule: cats are always right."),
            l("Mochi", "Second rule: see rule one."),
            l("You", "This is going to be interesting..."),
            l("Mochi", "Now scratch behind my ears. It helps me think."),
            l("You", "I appreciate it, but I'll figure it out myself."),
            l("Mochi", "Hmph. Stubborn, just like your grandma."),
            l("You", "I start cleaning. Mochi watches from a shelf."),
            l("Mochi", "...You missed a spot."),
            l("You", "Okay, MAYBE I need a little help."),
            l("???", "The bell chimes. Your first customer arrives."),
            l("Customer", "Oh! Is this place open? It's so charming!"),
            l("Mochi", "*purrs loudly*"),
            l("You", "Welcome to Paws & Whiskers! Please, sit down."),
            l("Customer", "What a sweet cat! Can I pet them?"),
            l("Mochi", "(to you) I like this one. Extra cream."),
            l("You", "And so begins my new life as a cat cafe owner."),
            l("You", "With a talking cat as my business partner."),
            l("Mochi", "Senior partner. Don't forget it."),
            l("???", "Paws & Whiskers Cafe — NOW OPEN"),
            l("???", "THE END — Press ESC"),
        ]
    }
}

impl Game for CatCafe {
    fn init(&mut self, engine: &mut Engine) {
        if let Err(e) = engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/women.png"), "bg")
        { eprintln!("{e}"); }
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
            engine.set_title(&format!(
                "Cat Cafe Chronicles | Rep: {} | FPS: {:.0}",
                self.reputation,
                self.fps.fps()
            ));
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
                self.chose_mochi = true;
                self.reputation += 10;
                self.choosing = false;
                self.show(engine, 15);
            } else if k2 {
                self.chose_mochi = false;
                self.reputation += 5;
                self.choosing = false;
                self.show(engine, 20);
            }
        } else if space {
            self.advance(engine);
        }
    }
}

impl CatCafe {
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
            120.0,
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
            self.show_choice(engine);
            return;
        }
        let next = if self.idx == 19 || self.idx == 24 {
            25
        } else {
            next
        };
        self.show(engine, next);
    }

    fn show_choice(&mut self, engine: &mut Engine) {
        self.choosing = true;
        let txt = "How do you respond to Mochi?\n\
                   1: Accept Mochi as your mentor\n\
                   2: Try to figure it out alone";
        engine.get_text_system().remove_text(self.tw_id);
        self.tw_id = engine.get_text_system().add_text(
            txt,
            TextSpeed::Instant,
            60.0,
            H - 120.0,
            TextStyle::new(64.0),
            PunctuationConfig::default(),
        );
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
    let game = CatCafe {
        script: CatCafe::script(),
        idx: 0,
        tw_id: 0,
        vfx: VfxRenderer::new(),
        fps: FpsCounter::new(),
        title_t: 0.0,
        ended: false,
        choosing: false,
        chose_mochi: false,
        reputation: 0,
    };
    let config = WindowConfig {
        title: "Cat Cafe Chronicles".into(),
        width: W as u32,
        height: H as u32,
        resizable: true,
        fullscreen: true,
        vsync: true,
        background_color: Color::new(0.12, 0.06, 0.02, 1.0),
        language: Language::resolve("en_us").unwrap(),
    };
    app::run(config, Box::new(game)).expect("Failed to run");
}
