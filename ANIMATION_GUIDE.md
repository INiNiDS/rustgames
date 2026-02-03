# Sprite Animation System - Complete Guide

## Overview

The RsGames engine features a production-grade sprite animation system that uses **UV coordinate manipulation** for efficient animated sprites. No texture swapping needed - just update UV coordinates!

## How It Works

### The Core Concept

Traditional approach (slow):
```rust
// BAD: Swapping entire textures every frame
texture = animation_frames[current_frame];  // Uploads new texture to GPU
```

Our approach (fast):
```rust
// GOOD: Just update UV coordinates
uv_rect = animation.current_uv();  // Returns Vec4 (16 bytes)
// Texture stays on GPU, only UV coords change!
```

### Architecture

1. **Sprite Sheet**: A single texture containing all animation frames in a grid
2. **SpriteAnimation**: State machine that tracks which frame to show
3. **UV Coordinates**: Tell the GPU which part of the texture to display
4. **SpriteInstance**: Contains UV rect, sent to GPU via instanced rendering

## Step-by-Step Usage Guide

### Step 1: Prepare Your Sprite Sheet

Your sprite sheet should be organized in a grid:

```
┌─────┬─────┬─────┬─────┐
│ F0  │ F1  │ F2  │ F3  │  Row 0
├─────┼─────┼─────┼─────┤
│ F4  │ F5  │ F6  │ F7  │  Row 1
├─────┼─────┼─────┼─────┤
│ F8  │ F9  │ F10 │ F11 │  Row 2
└─────┴─────┴─────┴─────┘
  C0    C1    C2    C3
```

Example: A 4x3 grid with 12 animation frames

### Step 2: Load the Texture

```rust
// In your Game::init()
engine.get_texture_controller().load_texture(
    include_bytes!("../assets/character_spritesheet.png"),
    "character"
);
```

### Step 3: Create the Animation

```rust
use rustgames::graphics::{SpriteAnimation, AnimationMode};

// Create from grid layout
let walk_animation = SpriteAnimation::from_grid(
    4,                      // columns in sprite sheet
    3,                      // rows in sprite sheet
    12,                     // total frames to use
    10.0,                   // frames per second
    AnimationMode::Loop,    // animation mode
);
```

**Animation Modes:**
- `AnimationMode::Loop` - Cycles continuously (0→1→2→...→11→0→...)
- `AnimationMode::PlayOnce` - Plays once and stops on last frame
- `AnimationMode::PingPong` - Plays forward then backward (0→11→0→11→...)

### Step 4: Store Animation Per Entity

```rust
struct MyEntity {
    position: Vec2,
    animation: SpriteAnimation,
    // ... other fields
}

impl MyEntity {
    fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            animation: SpriteAnimation::from_grid(4, 3, 12, 10.0, AnimationMode::Loop),
        }
    }
}
```

### Step 5: Update Animation in Game Loop

```rust
impl Game for MyGame {
    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();
        
        // Update all entity animations
        for entity in &mut self.entities {
            entity.animation.update(delta);
        }
        
        // ... other game logic
    }
}
```

### Step 6: Render with Animated UV Coordinates

This is where the magic happens!

```rust
use rustgames::graphics::SpriteInstance;
use glam::{Vec2, Vec4};

impl Game for MyGame {
    fn update(&mut self, engine: &mut Engine) {
        // ... update logic ...
        
        // Render all entities with instanced rendering
        let texture_controller = engine.get_texture_controller();
        
        for entity in &self.entities {
            // Get current UV from animation (this changes each frame!)
            let uv = entity.animation.current_uv();
            
            // Create sprite instance with animated UV coordinates
            let instance = SpriteInstance::new(
                entity.position,        // World position
                Vec2::new(64.0, 64.0), // Size in pixels
                0.0,                    // Rotation (radians)
                uv,                     // ← ANIMATED UV from sprite sheet
                Vec4::ONE,              // White (no color tint)
            );
            
            // Add to batch (rendered in single draw call per texture)
            texture_controller.add_instance("character", instance);
        }
    }
}
```

## Complete Example: Stress Test with 10,000 Entities

See `examples/demo_stress.rs` for a full working example:

```rust
struct StressDemo {
    animations: Vec<SpriteAnimation>,
    positions: Vec<Vec2>,
    velocities: Vec<Vec2>,
}

impl Game for StressDemo {
    fn init(&mut self, engine: &mut Engine) {
        // Load sprite sheet
        engine.get_texture_controller().load_texture(
            include_bytes!("../src/mistral.png"),
            "stress_sprite"
        );
        
        // Create 10,000 entities with random animations
        for _ in 0..10_000 {
            let anim = SpriteAnimation::from_grid(
                2, 2, 4,
                5.0 + rand::random::<f32>() * 10.0, // Random FPS
                AnimationMode::Loop
            );
            self.animations.push(anim);
            // ... initialize positions, velocities
        }
    }

    fn update(&mut self, engine: &mut Engine) {
        let delta = engine.delta_time();
        
        // Update ALL animations
        for anim in &mut self.animations {
            anim.update(delta);
        }
        
        // Render ALL 10,000+ entities in a SINGLE draw call
        let texture_controller = engine.get_texture_controller();
        
        for i in 0..self.animations.len() {
            let uv = self.animations[i].current_uv();
            let instance = SpriteInstance::new(
                self.positions[i],
                Vec2::new(20.0, 20.0),
                0.0,
                uv,  // Animated UV
                Vec4::ONE,
            );
            texture_controller.add_instance("stress_sprite", instance);
        }
    }
}
```

## Advanced Animation Control

### Pause/Resume

```rust
animation.pause();   // Freeze animation
animation.resume();  // Continue animation
```

### Reset

```rust
animation.reset();  // Back to frame 0
```

### Jump to Specific Frame

```rust
animation.set_frame(5);  // Jump to frame 5
```

### Check Status

```rust
if animation.is_finished() {
    // Only true for AnimationMode::PlayOnce
    println!("Animation completed!");
}

let current = animation.current_frame_index();  // Get frame number
let total = animation.frame_count();            // Get total frames
```

## Performance Characteristics

### Memory Usage
- `SpriteAnimation`: ~40 bytes + 16 bytes per frame
- 10,000 animations with 12 frames each: ~2 MB

### CPU Cost
- Update: O(1) per animation
- UV fetch: O(1) - just array access
- No allocations after creation

### GPU Efficiency
- ✅ Single texture stays in GPU memory
- ✅ Only UV coordinates change (16 bytes per instance)
- ✅ All instances batched into single draw call per texture
- ✅ Hardware instancing = 1000x faster than naive approach

## UV Coordinate Math

Understanding UV coordinates:

```
UV Space (normalized 0.0 to 1.0):
┌─────────────────────┐ 1.0
│                     │
│   (0.25, 0.25)     │
│   ┌────────┐       │
│   │ Frame  │       │
│   │  UV    │       │
│   └────────┘       │
│   w: 0.25, h: 0.25 │
│                     │
└─────────────────────┘ 0.0
0.0                  1.0
```

For a 4x4 grid:
- Each frame width: 1.0 / 4 = 0.25
- Each frame height: 1.0 / 4 = 0.25
- Frame (1, 1) UV: (0.25, 0.25, 0.25, 0.25)

The shader does:
```wgsl
out.tex_coords = uv_rect.xy + vertex.tex_coords * uv_rect.zw;
```

This maps the quad's [0,1] UVs to the sprite sheet region!

## Common Patterns

### Multiple Animations Per Character

```rust
struct Character {
    idle_anim: SpriteAnimation,
    walk_anim: SpriteAnimation,
    attack_anim: SpriteAnimation,
    current_anim: AnimationType,
}

enum AnimationType {
    Idle,
    Walk,
    Attack,
}

impl Character {
    fn update(&mut self, delta: f32) {
        match self.current_anim {
            AnimationType::Idle => self.idle_anim.update(delta),
            AnimationType::Walk => self.walk_anim.update(delta),
            AnimationType::Attack => self.attack_anim.update(delta),
        }
    }
    
    fn current_uv(&self) -> Vec4 {
        match self.current_anim {
            AnimationType::Idle => self.idle_anim.current_uv(),
            AnimationType::Walk => self.walk_anim.current_uv(),
            AnimationType::Attack => self.attack_anim.current_uv(),
        }
    }
}
```

### Animation State Machine

```rust
impl Character {
    fn switch_animation(&mut self, new_anim: AnimationType) {
        if self.current_anim != new_anim {
            self.current_anim = new_anim;
            
            // Reset new animation to start
            match new_anim {
                AnimationType::Idle => self.idle_anim.reset(),
                AnimationType::Walk => self.walk_anim.reset(),
                AnimationType::Attack => self.attack_anim.reset(),
            }
        }
    }
}
```

### Varying Animation Speeds

```rust
// Fast enemy
let fast_anim = SpriteAnimation::from_grid(4, 4, 16, 20.0, AnimationMode::Loop);

// Slow enemy
let slow_anim = SpriteAnimation::from_grid(4, 4, 16, 5.0, AnimationMode::Loop);
```

## Integration with Instanced Rendering

The complete pipeline:

```
[Game Update]
    ↓
[Update all SpriteAnimations with delta_time]
    ↓
[For each entity: Get current_uv() from animation]
    ↓
[Create SpriteInstance with animated UV]
    ↓
[Add instance to TextureController batch]
    ↓
[Renderer batches all instances per texture]
    ↓
[SpriteRenderer uploads instance buffer to GPU (bytemuck)]
    ↓
[WGSL shader reads instance data]
    ↓
[Single draw_indexed() call renders all instances]
    ↓
[GPU transforms each sprite with UV rectangle]
```

## Debugging Tips

### Visualize Frame Numbers

```rust
println!("Entity {} is on frame {}/{}", 
    entity_id, 
    animation.current_frame_index(), 
    animation.frame_count()
);
```

### Check UV Values

```rust
let uv = animation.current_uv();
println!("UV: x={}, y={}, w={}, h={}", uv.x, uv.y, uv.z, uv.w);
```

### Verify Animation Timing

```rust
let fps = 10.0;
let frame_duration = 1.0 / fps;  // Should be 0.1 seconds per frame
```

## Best Practices

1. ✅ **One texture per sprite sheet** - Don't split animations across textures
2. ✅ **Power-of-2 dimensions** - Better GPU performance (256x256, 512x512, etc.)
3. ✅ **Consistent frame sizes** - All frames same dimensions in the grid
4. ✅ **Batch by texture** - Engine does this automatically
5. ✅ **Update once per frame** - Don't update animation multiple times per frame
6. ✅ **Reuse animations** - Clone animation state for multiple entities if needed

## Performance Validation

Run the stress test:
```bash
cargo run --example demo_stress --release
```

Expected results on modern hardware:
- 10,000+ entities at 60 FPS
- Single draw call per texture
- ~1-2ms frame time
- Smooth camera shake
- All entities fully animated

Press UP arrow to add more entities and see the limits of your GPU!

## Summary

The sprite animation system provides:
- ✅ Zero texture uploads per frame
- ✅ Hardware instancing support
- ✅ Multiple animation modes (Loop, PlayOnce, PingPong)
- ✅ Frame-accurate timing
- ✅ Pause/resume/reset controls
- ✅ Memory efficient (~40 bytes per animation)
- ✅ CPU efficient (O(1) update)
- ✅ Production-ready for 10,000+ entities

Ready to make your sprites come alive! 🎮
