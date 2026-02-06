use rustgames::prelude::*;
use rustgames::core::app;
use glam::{Vec2, Vec4};

struct AnimationDemo {
    loop_animation: SpriteAnimation,
    play_once_animation: SpriteAnimation,
    ping_pong_animation: SpriteAnimation,
    current_mode: AnimationModeDemo,
    fps_counter: FpsCounter,
    info_update_timer: f32,
    is_paused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AnimationModeDemo {
    Loop,
    PlayOnce,
    PingPong,
}

impl AnimationModeDemo {
    fn name(&self) -> &str {
        match self {
            AnimationModeDemo::Loop => "LOOP",
            AnimationModeDemo::PlayOnce => "PLAY ONCE",
            AnimationModeDemo::PingPong => "PING PONG",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            AnimationModeDemo::Loop => "Cycles continuously: 0→1→2→3→0→1→...",
            AnimationModeDemo::PlayOnce => "Plays once and stops: 0→1→2→3 [STOP]",
            AnimationModeDemo::PingPong => "Plays forward then backward: 0→1→2→3→2→1→0→...",
        }
    }
}

impl Game for AnimationDemo {
    fn init(&mut self, engine: &mut Engine) {
        println!("╔════════════════════════════════════════════════════════════╗");
        println!("║         SPRITE ANIMATION SYSTEM DEMONSTRATION              ║");
        println!("╚════════════════════════════════════════════════════════════╝");
        println!();
        println!("This demo shows how the sprite animation system works:");
        println!("  • UV-based animation (no texture swapping!)");
        println!("  • Three animation modes: Loop, PlayOnce, PingPong");
        println!("  • Real-time frame tracking");
        println!();
        println!("CONTROLS:");
        println!("  1     - Switch to LOOP mode");
        println!("  2     - Switch to PLAY ONCE mode");
        println!("  3     - Switch to PING PONG mode");
        println!("  R     - Reset current animation");
        println!("  SPACE - Pause/Resume animation");
        println!("  ESC   - Exit");
        println!();

        engine.get_texture_controller().load_texture(
            include_bytes!("../src/mistral.png"),
            "animation_sheet"
        );

        engine.get_camera().set_zoom(300.0);
        
        println!("✓ Animation system initialized");
        println!("✓ Sprite sheet loaded (2x2 grid, 4 frames)");
        println!("✓ Starting in LOOP mode");
        println!();
        self.print_mode_info();

        engine.get_audio_system().load_sound("perdej", "/home/ininids/RustroverProjects/rsgames/src/sound_03850.mp3");
    }

    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();

        self.fps_counter.update(delta);
        self.info_update_timer += delta;

        self.get_current_animation_mut().update(delta);

        self.handle_input(engine);

        self.render_sprite(engine);

        if self.info_update_timer >= 0.1 {
            self.info_update_timer = 0.0;
            self.update_window_title(engine);
        }
    }
}

impl AnimationDemo {
    fn new() -> Self {
        Self {
            loop_animation: SpriteAnimation::from_grid(2, 2, 4, 8.0, AnimationMode::Loop),
            play_once_animation: SpriteAnimation::from_grid(2, 2, 4, 8.0, AnimationMode::PlayOnce),
            ping_pong_animation: SpriteAnimation::from_grid(2, 2, 4, 8.0, AnimationMode::PingPong),
            current_mode: AnimationModeDemo::Loop,
            fps_counter: FpsCounter::new(),
            info_update_timer: 0.0,
            is_paused: false,
        }
    }
    
    fn get_current_animation_mut(&mut self) -> &mut SpriteAnimation {
        match self.current_mode {
            AnimationModeDemo::Loop => &mut self.loop_animation,
            AnimationModeDemo::PlayOnce => &mut self.play_once_animation,
            AnimationModeDemo::PingPong => &mut self.ping_pong_animation,
        }
    }
    
    fn handle_input(&mut self, engine: &mut Engine) {
        let event_queue = engine.get_event_queue();

        if event_queue.was_key_just_pressed(KeyCode::Digit1) {
            self.switch_mode(AnimationModeDemo::Loop);
        }
        if event_queue.was_key_just_pressed(KeyCode::Digit2) {
            self.switch_mode(AnimationModeDemo::PlayOnce);
        }
        if event_queue.was_key_just_pressed(KeyCode::Digit3) {
            self.switch_mode(AnimationModeDemo::PingPong);
        }

        if event_queue.was_key_just_pressed(KeyCode::KeyR) {
            let current_anim = self.get_current_animation_mut();
            current_anim.reset();
            println!("→ Animation reset to frame 0");
        }

        if event_queue.was_key_just_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
            if self.is_paused {
                self.get_current_animation_mut().pause();
                println!("⏸ Animation paused");
            } else {
                self.get_current_animation_mut().resume();
                println!("▶ Animation resumed");
            }
        }

        if event_queue.was_key_just_pressed(KeyCode::Escape) {
            println!();
            println!("Animation demo ended.");
            std::process::exit(0);
        }
    }
    
    fn switch_mode(&mut self, new_mode: AnimationModeDemo) {
        if self.current_mode != new_mode {
            self.current_mode = new_mode;
            println!();
            println!("═══════════════════════════════════════════════════════════");
            println!("Switched to: {}", new_mode.name());
            println!("═══════════════════════════════════════════════════════════");
            self.print_mode_info();
        }
    }
    
    fn print_mode_info(&self) {
        println!("Mode: {}", self.current_mode.name());
        println!("Description: {}", self.current_mode.description());
        println!();
    }
    
    fn render_sprite(&mut self, engine: &mut Engine) {
        let texture_controller = engine.get_texture_controller();
        let uv = self.get_current_animation_mut().current_uv();

        let size = texture_controller.get_texture("animation_sheet").unwrap().size;

        let instance = SpriteInstance::new(
            Vec2::ZERO,
            size,
            0.0,
            uv,
            Vec4::ONE,
        );

        texture_controller.add_instance("animation_sheet", instance);
    }
    
    fn update_window_title(&mut self, engine: &mut Engine) {
        let current_frame = self.get_current_animation_mut().current_frame_index();
        let frame_count = self.get_current_animation_mut().frame_count();
        let is_finished = self.get_current_animation_mut().is_finished();
        
        let frame_info = format!(
            "Frame: {}/{}",
            current_frame + 1,
            frame_count
        );
        
        let status = if is_finished {
            "[FINISHED]"
        } else {
            "[PLAYING]"
        };
        
        let title = format!(
            "Animation Demo | Mode: {} | {} {} | FPS: {:.0}",
            self.current_mode.name(),
            frame_info,
            status,
            self.fps_counter.fps()
        );
        
        engine.set_title(&title);
    }
}

fn main() {
    println!("Initializing animation demo...");
    println!();
    
    let game = AnimationDemo::new();

    let window_config = WindowConfig {
        title: "Demo: animation".to_string(),
        width: 2560,
        height: 1440,
        resizable: false,
        fullscreen: true,
        vsync: true,
        background_color: Color::WHITE,
    };

    app::run(window_config, Box::new(game))
        .expect("Failed to run animation test");
}
