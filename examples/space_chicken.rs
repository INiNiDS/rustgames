//! **Space Chicken: The Last Nugget** — A 2D story adventure demo.
//!
//! A brave space chicken must navigate through cosmic scenes, collecting
//! power-ups and making choices. The entire game is built exclusively with
//! the `rustgames` engine: sprites, typewriter text, animations, particles,
//! camera shake, and timeline effects.
//!
//! Controls:
//!   Arrow keys / WASD — Move the chicken
//!   SPACE — Interact / advance dialogue
//!   ENTER — Confirm choice
//!   ESC   — Quit

use glam::Vec2;
use rustgames::core::app;
use rustgames::prelude::*;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const SCREEN_W: f32 = 1280.0;
const SCREEN_H: f32 = 720.0;
const MOVE_SPEED: f32 = 200.0;
const NUGGET_SIZE: f32 = 30.0;
const PLAYER_SIZE: f32 = 60.0;
const ENEMY_SIZE: f32 = 40.0;

// ---------------------------------------------------------------------------
// Game phases
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Intro,
    Explore,
    BossFight,
    Ending,
}

// ---------------------------------------------------------------------------
// Nugget (collectible)
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Nugget {
    pos: Vec2,
    collected: bool,
    bob_time: f32,
}

impl Nugget {
    fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            collected: false,
            bob_time: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Enemy
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct Enemy {
    pos: Vec2,
    dir: Vec2,
    speed: f32,
    alive: bool,
}

impl Enemy {
    fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            dir: Vec2::new(dx, dy).normalize_or_zero(),
            speed: 80.0,
            alive: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Boss
// ---------------------------------------------------------------------------

struct Boss {
    pos: Vec2,
    health: i32,
    max_health: i32,
    phase_timer: f32,
}

impl Boss {
    fn new() -> Self {
        Self {
            pos: Vec2::new(300.0, -200.0),
            health: 5,
            max_health: 5,
            phase_timer: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Main game struct
// ---------------------------------------------------------------------------

struct SpaceChicken {
    phase: Phase,
    player_pos: Vec2,
    player_vel: Vec2,
    nuggets: Vec<Nugget>,
    enemies: Vec<Enemy>,
    boss: Boss,
    score: i32,
    vfx: VfxRenderer,
    fps: FpsCounter,
    title_timer: f32,
    dialogue_id: usize,
    dialogue_shown: bool,
    intro_done: bool,
    boss_defeated: bool,
    ending_timer: f32,
    flash_cooldown: f32,
    story_step: usize,
    hp: u32,
}

// ---------------------------------------------------------------------------
// Game trait implementation
// ---------------------------------------------------------------------------

impl Game for SpaceChicken {
    fn init(&mut self, engine: &mut Engine) {
        engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/mistral.png"), "chicken");
        engine
            .get_texture_controller()
            .load_texture(include_bytes!("../src/static/textures/OIP-475081084.jpg"), "bg");

        engine.get_camera().set_zoom(1.0);
        self.show_dialogue(engine, "Cluck cluck! I am Captain Feathers!");
        self.play_intro_animation(engine);
    }

    fn update(&mut self, engine: &mut Engine) {
        let dt = engine.delta_time();
        self.fps.update(dt);
        self.vfx.update(dt);
        self.title_timer += dt;

        match self.phase {
            Phase::Intro => self.update_intro(engine, dt),
            Phase::Explore => self.update_explore(engine, dt),
            Phase::BossFight => self.update_boss(engine, dt),
            Phase::Ending => self.update_ending(engine, dt),
        }

        self.draw_scene(engine);
        self.update_title(engine);
        if self.hp == 0 {
            self.phase = Phase::Ending;
            self.show_dialogue(
                engine,
                "Oh no! Captain Feathers has been defeated! - GAME OVER - Press ESC",
            );
        }
    }

    fn handle_update(&mut self, engine: &mut Engine) {
        self.handle_input(engine);
    }
}

// ---------------------------------------------------------------------------
// Intro phase
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn update_intro(&mut self, engine: &mut Engine, _dt: f32) {
        if self.intro_done {
            return;
        }
        if engine.get_text_system().is_complete(self.dialogue_id) && !self.dialogue_shown {
            self.dialogue_shown = true;
        }
    }

    fn advance_intro(&mut self, engine: &mut Engine) {
        if !self.dialogue_shown && !engine.get_text_system().is_complete(self.dialogue_id) {
            engine.get_text_system().skip(self.dialogue_id);
            self.dialogue_shown = true;
            return;
        }

        self.story_step += 1;
        match self.story_step {
            1 => self.show_dialogue(engine, "The evil Fox Empire stole all the golden nuggets!"),
            2 => self.show_dialogue(
                engine,
                "I must collect them before dinner time... I mean, before they conquer the galaxy!",
            ),
            3 => self.show_dialogue(engine, "WASD to move, SPACE to interact. Let's go!"),
            _ => {
                self.intro_done = true;
                self.phase = Phase::Explore;
                self.spawn_nuggets();
                self.spawn_enemies();
                self.show_dialogue(
                    engine,
                    "Collect all the golden nuggets! Watch out for fox scouts!",
                );
            }
        }
        self.dialogue_shown = false;
    }
}

// ---------------------------------------------------------------------------
// Explore phase
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn update_explore(&mut self, engine: &mut Engine, dt: f32) {
        self.move_player(dt);
        self.update_nuggets(dt);
        self.update_enemies(engine, dt);
        self.check_nugget_collection(engine);
        self.check_enemy_collision(engine);

        let remaining = self.nuggets.iter().filter(|n| !n.collected).count();
        if remaining == 0 && !self.boss_defeated {
            self.phase = Phase::BossFight;
            self.boss = Boss::new();
            self.show_dialogue(
                engine,
                "All nuggets collected! But wait... BOSS FOX GENERAL appears!",
            );
            self.dialogue_shown = false;
            self.play_boss_entrance(engine);
        }
    }

    fn spawn_nuggets(&mut self) {
        self.nuggets = vec![
            Nugget::new(-400.0, 100.0),
            Nugget::new(200.0, -150.0),
            Nugget::new(-100.0, 250.0),
            Nugget::new(350.0, 200.0),
            Nugget::new(-300.0, -200.0),
        ];
    }

    fn spawn_enemies(&mut self) {
        self.enemies = vec![
            Enemy::new(150.0, 0.0, 0.0, 1.0),
            Enemy::new(-200.0, 100.0, 1.0, 0.0),
            Enemy::new(300.0, -100.0, -1.0, 1.0),
        ];
    }

    fn update_nuggets(&mut self, dt: f32) {
        for nugget in &mut self.nuggets {
            if !nugget.collected {
                nugget.bob_time += dt * 3.0;
            }
        }
    }

    fn update_enemies(&mut self, engine: &mut Engine, dt: f32) {
        for enemy in &mut self.enemies {
            if !enemy.alive {
                continue;
            }
            enemy.pos += enemy.dir * enemy.speed * dt;
            if enemy.pos.x.abs() > 500.0 {
                enemy.dir.x = -enemy.dir.x;
            }
            if enemy.pos.y.abs() > 350.0 {
                enemy.dir.y = -enemy.dir.y;
            }
        }
        let _ = engine;
    }

    fn check_nugget_collection(&mut self, engine: &mut Engine) {
        let mut collected_idx = None;
        for (i, nugget) in self.nuggets.iter().enumerate() {
            if nugget.collected {
                continue;
            }
            let dist = (self.player_pos - nugget.pos).length();
            if dist < (PLAYER_SIZE + NUGGET_SIZE) / 2.0 {
                collected_idx = Some(i);
                break;
            }
        }
        if let Some(i) = collected_idx {
            let pos = self.nuggets[i].pos;
            self.nuggets[i].collected = true;
            self.score += 100;
            self.vfx
                .add_effect(VfxEffect::Emitter(EmitterConfig::sparkles(pos)));
            engine.get_camera().add_trauma(0.15);
            let msg = format!("Nugget collected! Score: {} Cluck!", self.score);
            self.show_dialogue(engine, &msg);
        }
    }

    fn check_enemy_collision(&mut self, engine: &mut Engine) {
        if self.flash_cooldown > 0.0 {
            return;
        }
        for enemy in &self.enemies {
            if !enemy.alive {
                continue;
            }
            let dist = (self.player_pos - enemy.pos).length();
            if dist < (PLAYER_SIZE + ENEMY_SIZE) / 2.0 {
                self.score = (self.score - 50).max(0);
                self.vfx.add_effect(VfxEffect::Flash {
                    color: Color::RED,
                    duration: 0.3,
                });
                engine.get_camera().add_trauma(0.4);
                self.flash_cooldown = 1.0;
                self.show_dialogue(engine, "Ouch! A fox scout got me! -50 points! BAWK!");
                self.hp -= 1;
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Boss fight phase
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn update_boss(&mut self, engine: &mut Engine, dt: f32) {
        self.move_player(dt);
        self.boss.phase_timer += dt;

        let orbit_x = self.boss.phase_timer.cos() * 200.0;
        let orbit_y = self.boss.phase_timer.sin() * 150.0;
        self.boss.pos = Vec2::new(orbit_x, orbit_y);

        let dist = (self.player_pos - self.boss.pos).length();
        if dist < 80.0 && self.flash_cooldown <= 0.0 {
            self.hit_boss(engine);
        }
    }

    fn hit_boss(&mut self, engine: &mut Engine) {
        self.boss.health -= 1;
        self.flash_cooldown = 0.5;
        self.vfx.add_effect(VfxEffect::Flash {
            color: Color::YELLOW,
            duration: 0.2,
        });
        engine.get_camera().add_trauma(0.5);
        self.vfx
            .add_effect(VfxEffect::Emitter(EmitterConfig::explosion(self.boss.pos)));

        if self.boss.health <= 0 {
            self.boss_defeated = true;
            self.score += 500;
            self.phase = Phase::Ending;
            self.ending_timer = 0.0;
            self.show_dialogue(
                engine,
                "The Fox General is defeated! The galaxy nuggets are safe!",
            );
            self.play_victory_animation(engine);
        } else {
            let msg = format!(
                "Take that! Boss HP: {}/{}",
                self.boss.health, self.boss.max_health
            );
            self.show_dialogue(engine, &msg);
        }
    }
}

// ---------------------------------------------------------------------------
// Ending phase
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn update_ending(&mut self, engine: &mut Engine, dt: f32) {
        self.ending_timer += dt;
        if self.ending_timer > 3.0 && self.story_step < 100 {
            self.story_step = 100;
            self.show_dialogue(
                engine,
                &format!(
                    "Captain Feathers saved the galaxy! Final Score: {} - THE END - Press ESC",
                    self.score
                ),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Drawing
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn draw_scene(&mut self, engine: &mut Engine) {
        engine.get_texture_controller().use_texture(
            "bg",
            Vec2::new(SCREEN_W * 2.0, SCREEN_H * 2.0),
            Vec2::ZERO,
            0.0,
            1.0,
        );

        self.draw_nuggets(engine);
        self.draw_enemies(engine);
        self.draw_player(engine);
        self.draw_boss(engine);
        self.draw_particles(engine);
    }

    fn draw_player(&mut self, engine: &mut Engine) {
        let visual = engine.get_animation_system().evaluate(
            VisualState {
                position: self.player_pos,
                ..Default::default()
            },
            Vec2::splat(PLAYER_SIZE),
            None,
        );
        engine.get_texture_controller().use_texture(
            "chicken",
            Vec2::splat(PLAYER_SIZE) * visual.scale,
            visual.position,
            visual.rotation,
            visual.opacity,
        );
    }

    fn draw_nuggets(&mut self, engine: &mut Engine) {
        for nugget in &self.nuggets {
            if nugget.collected {
                continue;
            }
            let bob = nugget.bob_time.sin() * 8.0;
            let pos = nugget.pos + Vec2::new(0.0, bob);
            let instance = SpriteInstance::new(
                pos,
                Vec2::splat(NUGGET_SIZE),
                0.0,
                glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
                glam::Vec4::new(1.0, 0.85, 0.0, 1.0),
            );
            engine
                .get_texture_controller()
                .add_instance("chicken", instance);
        }
    }

    fn draw_enemies(&mut self, engine: &mut Engine) {
        for enemy in &self.enemies {
            if !enemy.alive {
                continue;
            }
            let instance = SpriteInstance::new(
                enemy.pos,
                Vec2::splat(ENEMY_SIZE),
                0.0,
                glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
                glam::Vec4::new(1.0, 0.3, 0.0, 0.9),
            );
            engine
                .get_texture_controller()
                .add_instance("chicken", instance);
        }
    }

    fn draw_boss(&mut self, engine: &mut Engine) {
        if self.phase != Phase::BossFight {
            return;
        }
        let scale = 1.0 + (self.boss.phase_timer * 2.0).sin() * 0.1;
        let instance = SpriteInstance::new(
            self.boss.pos,
            Vec2::splat(90.0 * scale),
            self.boss.phase_timer * 0.5,
            glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
            glam::Vec4::new(0.8, 0.0, 0.0, 1.0),
        );
        engine
            .get_texture_controller()
            .add_instance("chicken", instance);
    }

    fn draw_particles(&mut self, engine: &mut Engine) {
        let frame = self.vfx.build_frame();
        for inst in &frame.particle {
            engine
                .get_texture_controller()
                .add_instance("chicken", *inst);
        }
    }
}

// ---------------------------------------------------------------------------
// Input handling
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn handle_input(&mut self, engine: &mut Engine) {
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;
        let space_pressed;
        let escape_pressed;

        {
            let eq = engine.get_event_queue();
            if eq.is_key_pressed(KeyCode::KeyW) || eq.is_key_pressed(KeyCode::ArrowUp) {
                dy -= 1.0;
            }
            if eq.is_key_pressed(KeyCode::KeyS) || eq.is_key_pressed(KeyCode::ArrowDown) {
                dy += 1.0;
            }
            if eq.is_key_pressed(KeyCode::KeyA) || eq.is_key_pressed(KeyCode::ArrowLeft) {
                dx -= 1.0;
            }
            if eq.is_key_pressed(KeyCode::KeyD) || eq.is_key_pressed(KeyCode::ArrowRight) {
                dx += 1.0;
            }
            space_pressed = eq.was_key_just_pressed(KeyCode::Space);
            escape_pressed = eq.was_key_just_pressed(KeyCode::Escape);
        }

        let dir = Vec2::new(dx, dy).normalize_or_zero();
        self.player_vel = dir * MOVE_SPEED;

        if space_pressed && self.phase == Phase::Intro {
            self.advance_intro(engine);
        }

        if escape_pressed {
            std::process::exit(0);
        }
    }

    fn move_player(&mut self, dt: f32) {
        self.player_pos += self.player_vel * dt;
        self.player_pos.x = self
            .player_pos
            .x
            .clamp(-SCREEN_W / 2.0 + 30.0, SCREEN_W / 2.0 - 30.0);
        self.player_pos.y = self
            .player_pos
            .y
            .clamp(-SCREEN_H / 2.0 + 30.0, SCREEN_H / 2.0 - 30.0);
        self.flash_cooldown = (self.flash_cooldown - 0.016).max(0.0);
    }
}

// ---------------------------------------------------------------------------
// Animation helpers
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn play_intro_animation(&mut self, engine: &mut Engine) {
        let timeline = TimelineBuilder::new()
            .parallel(vec![
                (Animation::FadeIn { duration: 1.5 }, Easing::EaseOut),
                (
                    Animation::Scale {
                        from: 0.0,
                        to: 1.0,
                        duration: 1.2,
                    },
                    Easing::Elastic,
                ),
            ])
            .build();
        engine.get_animation_system().start_timeline(timeline);
    }

    fn play_boss_entrance(&mut self, engine: &mut Engine) {
        let timeline = TimelineBuilder::new()
            .single(
                Animation::Shake {
                    intensity: 15.0,
                    duration: 1.0,
                },
                Easing::Linear,
            )
            .build();
        engine.get_animation_system().start_timeline(timeline);
        engine.get_camera().add_trauma(0.8);
    }

    fn play_victory_animation(&mut self, engine: &mut Engine) {
        let timeline = TimelineBuilder::new()
            .parallel(vec![
                (
                    Animation::Scale {
                        from: 1.0,
                        to: 2.0,
                        duration: 0.5,
                    },
                    Easing::EaseOut,
                ),
                (
                    Animation::Rotate {
                        from: 0.0,
                        to: std::f32::consts::TAU,
                        duration: 1.0,
                    },
                    Easing::EaseInOut,
                ),
            ])
            .single(
                Animation::Scale {
                    from: 2.0,
                    to: 1.0,
                    duration: 0.5,
                },
                Easing::Bounce,
            )
            .build();
        engine.get_animation_system().start_timeline(timeline);
        self.vfx
            .add_effect(VfxEffect::Emitter(EmitterConfig::explosion(Vec2::ZERO)));
        self.vfx
            .add_effect(VfxEffect::Emitter(EmitterConfig::sparkles(Vec2::new(
                100.0, 50.0,
            ))));
        self.vfx
            .add_effect(VfxEffect::Emitter(EmitterConfig::sparkles(Vec2::new(
                -100.0, -50.0,
            ))));
    }
}

// ---------------------------------------------------------------------------
// Dialogue helpers
// ---------------------------------------------------------------------------

impl SpaceChicken {
    fn show_dialogue(&mut self, engine: &mut Engine, text: &str) {
        engine.get_text_system().remove_text(self.dialogue_id);
        self.dialogue_id = engine.get_text_system().add_text(
            text,
            TextSpeed::Medium,
            40.0,
            40.0,
            TextStyle::new(64.0),
            PunctuationConfig::default(),
        );
    }

    fn update_title(&mut self, engine: &mut Engine) {
        if self.title_timer >= 0.5 {
            self.title_timer = 0.0;
            let phase_name = match self.phase {
                Phase::Intro => "Intro",
                Phase::Explore => "Explore",
                Phase::BossFight => "BOSS FIGHT",
                Phase::Ending => "Victory!",
            };
            engine.set_title(&format!(
                "Space Chicken | {} | Score: {} | FPS: {:.0}",
                phase_name,
                self.score,
                self.fps.fps(),
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    let game = SpaceChicken {
        phase: Phase::Intro,
        player_pos: Vec2::ZERO,
        player_vel: Vec2::ZERO,
        nuggets: Vec::new(),
        enemies: Vec::new(),
        boss: Boss::new(),
        score: 0,
        vfx: VfxRenderer::new(),
        fps: FpsCounter::new(),
        title_timer: 0.0,
        dialogue_id: 0,
        dialogue_shown: false,
        intro_done: false,
        boss_defeated: false,
        ending_timer: 0.0,
        flash_cooldown: 0.0,
        story_step: 0,
        hp: 5,
    };

    let config = WindowConfig {
        title: "Space Chicken: The Last Nugget".to_string(),
        width: SCREEN_W as u32,
        height: SCREEN_H as u32,
        resizable: true,
        fullscreen: false,
        vsync: true,
        background_color: Color::new(0.05, 0.02, 0.15, 1.0),
    };

    app::run(config, Box::new(game)).expect("Failed to run Space Chicken");
}
