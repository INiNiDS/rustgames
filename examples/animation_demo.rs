/// animation_demo.rs - Interactive demonstration of the sprite animation system
/// 
/// This example showcases all three animation modes and allows switching between them
/// to understand how the sprite animation system works in practice.
///
/// ## What This Demo Shows:
/// - How to create animations from sprite sheets
/// - Loop, PlayOnce, and PingPong animation modes
/// - Real-time mode switching
/// - Animation state (frame number, finished status)
/// - UV coordinate updates for sprite sheet rendering

use rustgames::prelude::*;
use rustgames::core::{app, FpsCounter};
use rustgames::graphics::{SpriteAnimation, AnimationMode, SpriteInstance};
use rustgames::window::KeyCode;
use glam::{Vec2, Vec4};

struct AnimationDemo {
    // Three different animations to demonstrate each mode
    loop_animation: SpriteAnimation,
    play_once_animation: SpriteAnimation,
    ping_pong_animation: SpriteAnimation,
    
    // Current active mode
    current_mode: AnimationModeDemo,
    
    // Display info
    fps_counter: FpsCounter,
    info_update_timer: f32,
    
    // Pause state
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
        
        // Load a texture that will be our sprite sheet
        // The mistral.png is organized as a 2x2 grid (4 frames)
        engine.get_texture_controller().load_texture(
            include_bytes!("../src/mistral.png"),
            "animation_sheet"
        );
        
        // Set up camera
        engine.get_camera_controller().set_zoom(200.0);
        
        println!("✓ Animation system initialized");
        println!("✓ Sprite sheet loaded (2x2 grid, 4 frames)");
        println!("✓ Starting in LOOP mode");
        println!();
        self.print_mode_info();
    }

    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();
        
        // Update FPS counter
        self.fps_counter.update(delta);
        self.info_update_timer += delta;
        
        // Update the active animation
        self.get_current_animation_mut().update(delta);
        
        // Handle keyboard input
        self.handle_input(engine);
        
        // Render the animated sprite
        self.render_sprite(engine);
        
        // Update window title with animation info
        if self.info_update_timer >= 0.1 {
            self.info_update_timer = 0.0;
            self.update_window_title(engine);
        }
    }
}

impl AnimationDemo {
    fn new() -> Self {
        // Create three animations, one for each mode
        // Using a 2x2 grid sprite sheet with 4 frames at 8 FPS
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
        
        // Switch animation modes
        if event_queue.was_key_just_pressed(KeyCode::Digit1) {
            self.switch_mode(AnimationModeDemo::Loop);
        }
        if event_queue.was_key_just_pressed(KeyCode::Digit2) {
            self.switch_mode(AnimationModeDemo::PlayOnce);
        }
        if event_queue.was_key_just_pressed(KeyCode::Digit3) {
            self.switch_mode(AnimationModeDemo::PingPong);
        }
        
        // Reset animation
        if event_queue.was_key_just_pressed(KeyCode::KeyR) {
            let current_anim = self.get_current_animation_mut();
            current_anim.reset();
            println!("→ Animation reset to frame 0");
        }
        
        // Pause/Resume
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
        
        // Exit
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
        // Get the current UV coordinates from the animation
        // This is the key: UV coordinates change, but the texture stays the same!
        let uv = self.get_current_animation_mut().current_uv();
        
        // Create a sprite instance with the animated UV coordinates
        let instance = SpriteInstance::new(
            Vec2::ZERO,              // Position (center of screen)
            Vec2::new(150.0, 150.0), // Size
            0.0,                     // Rotation
            uv,                      // UV coordinates from animation!
            Vec4::ONE,               // White color (no tint)
        );
        
        // Add to rendering batch
        let texture_controller = engine.get_texture_controller();
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
    
    app::run("Sprite Animation Demo", 800.0, 600.0, Box::new(game))
        .expect("Failed to run animation demo");
}
