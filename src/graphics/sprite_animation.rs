//! Sprite Animation System
//!
//! This module provides a UV-based sprite animation system for efficient animated sprites.
//! Instead of swapping textures, it updates UV coordinates to show different frames from
//! a single sprite sheet.
//!
//! # Quick Example
//!
//! ```rust,no_run
//! use rustgames::graphics::{SpriteAnimation, AnimationMode};
//! use glam::Vec4;
//!
//! // Create animation from a 4×4 sprite sheet grid
//! let mut animation = SpriteAnimation::from_grid(
//!     4,                      // columns
//!     4,                      // rows
//!     12,                     // number of frames to use
//!     10.0,                   // frames per second
//!     AnimationMode::Loop,    // animation mode
//! );
//!
//! // In your game loop:
//! // 1. Update animation
//! animation.update(delta_time);
//!
//! // 2. Get current UV coordinates
//! let uv = animation.current_uv();  // Returns Vec4(x, y, width, height)
//!
//! // 3. Use UV in your SpriteInstance for rendering
//! // let instance = SpriteInstance::new(position, size, rotation, uv, color);
//! ```
//!
//! # Animation Modes
//!
//! - **Loop**: Cycles continuously (0→1→2→3→0→...)
//! - **PlayOnce**: Plays once and stops (0→1→2→3 [STOP])
//! - **PingPong**: Plays forward then backward (0→1→2→3→2→1→0→...)
//!
//! # Performance
//!
//! - Memory: 40 bytes + 16 bytes per frame
//! - CPU: O(1) update, no allocations
//! - GPU: No texture uploads, just UV coordinates
//!
//! # See Also
//!
//! - `examples/animation_demo.rs` - Interactive demonstration
//! - `ANIMATION_GUIDE.md` - Complete usage guide
//! - `HOW_ANIMATION_WORKS.md` - Technical deep-dive

use glam::Vec4;

/// Animation playback mode for sprite animations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationMode {
    /// Play once and stop on the last frame
    PlayOnce,
    /// Loop continuously
    Loop,
    /// Play forward then backward continuously (ping-pong)
    PingPong,
}

/// Direction for ping-pong animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PingPongDirection {
    Forward,
    Backward,
}

/// A sprite animation that cycles through UV rectangles.
/// This allows animating sprites without swapping textures - just update UV coordinates.
/// Designed for sprite sheets/texture atlases.
#[derive(Debug, Clone)]
pub struct SpriteAnimation {
    /// UV rectangles for each frame (x, y, width, height in [0, 1] normalized coords)
    frames: Vec<Vec4>,
    
    /// Current frame index
    current_frame: usize,
    
    /// Time elapsed on current frame
    elapsed: f32,
    
    /// Duration per frame in seconds
    frame_duration: f32,
    
    /// Animation playback mode
    mode: AnimationMode,
    
    /// Direction for ping-pong mode
    ping_pong_direction: PingPongDirection,
    
    /// Whether the animation is paused
    paused: bool,
    
    /// Whether the animation has finished (for PlayOnce mode)
    finished: bool,
}

impl SpriteAnimation {
    /// Create a new sprite animation.
    /// 
    /// # Arguments
    /// * `frames` - UV rectangles for each frame
    /// * `fps` - Frames per second for playback
    /// * `mode` - Playback mode (Loop, PlayOnce, PingPong)
    pub fn new(frames: Vec<Vec4>, fps: f32, mode: AnimationMode) -> Self {
        assert!(!frames.is_empty(), "Animation must have at least one frame");
        assert!(fps > 0.0, "FPS must be positive");
        
        Self {
            frames,
            current_frame: 0,
            elapsed: 0.0,
            frame_duration: 1.0 / fps,
            mode,
            ping_pong_direction: PingPongDirection::Forward,
            paused: false,
            finished: false,
        }
    }
    
    /// Create animation from a grid-based sprite sheet.
    /// 
    /// # Arguments
    /// * `columns` - Number of columns in the sprite sheet
    /// * `rows` - Number of rows in the sprite sheet
    /// * `frame_count` - Total number of frames to use (may be less than columns * rows)
    /// * `fps` - Frames per second
    /// * `mode` - Playback mode
    pub fn from_grid(
        columns: usize,
        rows: usize,
        frame_count: usize,
        fps: f32,
        mode: AnimationMode,
    ) -> Self {
        assert!(columns > 0 && rows > 0, "Grid must have at least 1x1");
        assert!(frame_count > 0, "Must have at least one frame");
        assert!(
            frame_count <= columns * rows,
            "Frame count cannot exceed grid capacity"
        );
        
        let frame_width = 1.0 / columns as f32;
        let frame_height = 1.0 / rows as f32;
        
        let mut frames = Vec::with_capacity(frame_count);
        
        for i in 0..frame_count {
            let col = i % columns;
            let row = i / columns;
            
            let x = col as f32 * frame_width;
            let y = row as f32 * frame_height;
            
            frames.push(Vec4::new(x, y, frame_width, frame_height));
        }
        
        Self::new(frames, fps, mode)
    }
    
    /// Update the animation state.
    /// 
    /// # Arguments
    /// * `delta_time` - Time elapsed since last update in seconds
    pub fn update(&mut self, delta_time: f32) {
        if self.paused || self.finished {
            return;
        }
        
        self.elapsed += delta_time;
        
        while self.elapsed >= self.frame_duration {
            self.elapsed -= self.frame_duration;
            self.advance_frame();
        }
    }
    
    /// Advance to the next frame based on animation mode.
    fn advance_frame(&mut self) {
        match self.mode {
            AnimationMode::PlayOnce => {
                if self.current_frame < self.frames.len() - 1 {
                    self.current_frame += 1;
                } else {
                    self.finished = true;
                }
            }
            AnimationMode::Loop => {
                self.current_frame = (self.current_frame + 1) % self.frames.len();
            }
            AnimationMode::PingPong => {
                match self.ping_pong_direction {
                    PingPongDirection::Forward => {
                        if self.current_frame < self.frames.len() - 1 {
                            self.current_frame += 1;
                        } else {
                            self.ping_pong_direction = PingPongDirection::Backward;
                            if self.frames.len() > 1 {
                                self.current_frame -= 1;
                            }
                        }
                    }
                    PingPongDirection::Backward => {
                        if self.current_frame > 0 {
                            self.current_frame -= 1;
                        } else {
                            self.ping_pong_direction = PingPongDirection::Forward;
                            if self.frames.len() > 1 {
                                self.current_frame += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Get the current frame's UV rectangle.
    pub fn current_uv(&self) -> Vec4 {
        self.frames[self.current_frame]
    }
    
    /// Get the current frame index.
    pub fn current_frame_index(&self) -> usize {
        self.current_frame
    }
    
    /// Check if the animation has finished (only for PlayOnce mode).
    pub fn is_finished(&self) -> bool {
        self.finished
    }
    
    /// Pause the animation.
    pub fn pause(&mut self) {
        self.paused = true;
    }
    
    /// Resume the animation.
    pub fn resume(&mut self) {
        self.paused = false;
    }
    
    /// Reset the animation to the first frame.
    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed = 0.0;
        self.finished = false;
        self.ping_pong_direction = PingPongDirection::Forward;
    }
    
    /// Set the animation to a specific frame.
    pub fn set_frame(&mut self, frame: usize) {
        if frame < self.frames.len() {
            self.current_frame = frame;
            self.elapsed = 0.0;
        }
    }
    
    /// Get the total number of frames.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_loop() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 0.5, 0.5),
            Vec4::new(0.5, 0.0, 0.5, 0.5),
            Vec4::new(0.0, 0.5, 0.5, 0.5),
        ];
        
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
        
        // Check initial state
        assert_eq!(anim.current_frame_index(), 0);
        
        // Advance one frame (0.1 seconds at 10 FPS)
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        
        // Advance two more frames
        anim.update(0.2);
        assert_eq!(anim.current_frame_index(), 0); // Should loop back
        
        assert!(!anim.is_finished());
    }
    
    #[test]
    fn test_animation_play_once() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 0.5, 0.5),
            Vec4::new(0.5, 0.0, 0.5, 0.5),
        ];
        
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PlayOnce);
        
        anim.update(0.1); // Frame 1
        assert_eq!(anim.current_frame_index(), 1);
        assert!(!anim.is_finished());
        
        anim.update(0.1); // Should stay at frame 1
        assert_eq!(anim.current_frame_index(), 1);
        assert!(anim.is_finished());
    }
    
    #[test]
    fn test_animation_ping_pong() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
        ];
        
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PingPong);
        
        // Forward
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 2);
        
        // Backward
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 0);
        
        // Forward again
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1);
    }
    
    #[test]
    fn test_animation_from_grid() {
        let anim = SpriteAnimation::from_grid(4, 4, 8, 10.0, AnimationMode::Loop);
        
        assert_eq!(anim.frame_count(), 8);
        
        // Check first frame UV
        let first_uv = anim.current_uv();
        assert_eq!(first_uv.x, 0.0);
        assert_eq!(first_uv.y, 0.0);
        assert_eq!(first_uv.z, 0.25); // 1/4 width
        assert_eq!(first_uv.w, 0.25); // 1/4 height
    }
    
    #[test]
    fn test_animation_pause_resume() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
        ];
        
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
        
        anim.pause();
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 0); // Should not advance
        
        anim.resume();
        anim.update(0.1);
        assert_eq!(anim.current_frame_index(), 1); // Should advance
    }
    
    #[test]
    fn test_animation_reset() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(0.0, 0.0, 1.0, 1.0),
        ];
        
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::PlayOnce);
        
        anim.update(0.2); // Go to end
        assert!(anim.is_finished());
        
        anim.reset();
        assert_eq!(anim.current_frame_index(), 0);
        assert!(!anim.is_finished());
    }
}
