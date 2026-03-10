//! Extreme CPU stress tests for the engine's pure-logic subsystems.
//!
//! # Activation
//! These tests are **skipped by default**.  To run them, set the
//! `STRESS_TEST` environment variable to any non-empty value:
//!
//! ```sh
//! STRESS_TEST=1 cargo test -- --nocapture
//! ```
//!
//! # Design rationale
//! * **`std::hint::black_box`** – every computed value is fed through
//!   `black_box` so that LLVM cannot treat the loop body as dead code and
//!   silently eliminate it under `--release` / LTO.
//! * **Warm-up pass** – each test runs a short warm-up loop first to let the
//!   CPU reach its steady-state frequency and warm the branch predictor and
//!   i-cache before the timed measurement begins.
//! * **Hard time limits** – chosen conservatively so they comfortably pass on
//!   any modern desktop CPU in `cargo test` (debug) mode.  Under `--release`
//!   the actual time is typically 5-20× lower.
//! * **Headless** – no GPU, window, or audio resources are created; everything
//!   under test is pure CPU / RAM logic.

#![cfg(test)]

use std::hint::black_box;
use std::time::{Duration, Instant};

// ── helper ────────────────────────────────────────────────────────────────────

/// Returns `true` when the `STRESS_TEST` env var is set to a non-empty value.
fn stress_enabled() -> bool {
    std::env::var("STRESS_TEST").map_or(false, |v| !v.is_empty())
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. COLOR MATH BLITZ
//    500 000 lerp + to_u32 + from_rgba_u8 round-trips.
//    Limit: 200 ms — pure f32 arithmetic should be ≪ 10 ns/op on any modern
//    core.  Even at 2 ns/op × 500 000 × 3 ops = 3 ms; 200 ms gives ~67×
//    headroom for debug builds and slow CI runners.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_color_math_blitz() {
    if !stress_enabled() {
        return;
    }

    use rustgames::graphics::color::Color;

    const ITERATIONS: u32 = 500_000;
    const LIMIT: Duration = Duration::from_millis(200);

    // warm-up: 10 000 iterations (not timed)
    for i in 0u32..10_000 {
        let t = (i % 256) as f32 / 255.0;
        let c = black_box(Color::BLACK.lerp(Color::WHITE, t));
        black_box(c.to_u32());
    }

    let start = Instant::now();

    for i in 0u32..ITERATIONS {
        let t = (i % 256) as f32 / 255.0;
        // lerp: 4 mul_add + 4 clamp
        let lerped = black_box(Color::BLACK.lerp(Color::WHITE, t));
        // pack to u32: 4 clamp + 4 mul + 4 cast + 3 shift/or
        black_box(lerped.to_u32());
        // unpack from u8: 4 div
        let r = (i % 256) as u8;
        let g = ((i * 3) % 256) as u8;
        let b = ((i * 7) % 256) as u8;
        black_box(Color::from_rgba_u8(r, g, b, 255));
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_color_math_blitz: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({ITERATIONS} iterations). Possible regression in Color::lerp / to_u32 / from_rgba_u8."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. EASING PIPELINE STRESS
//    All 6 easing variants × 200 000 samples each = 1 200 000 `apply()` calls.
//    Limit: 150 ms — Bounce / Elastic involve sin/exp2 calls, so ~100 ns/op
//    worst-case × 1 200 000 = 120 ms; 150 ms gives a safe buffer.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_easing_all_variants() {
    if !stress_enabled() {
        return;
    }

    use rustgames::graphics::effects::animation::easing::Easing;

    const SAMPLES: u32 = 200_000;
    const LIMIT: Duration = Duration::from_millis(150);

    let variants = [
        Easing::Linear,
        Easing::EaseIn,
        Easing::EaseOut,
        Easing::EaseInOut,
        Easing::Bounce,
        Easing::Elastic,
    ];

    // warm-up
    for v in &variants {
        for i in 0u32..1_000 {
            black_box(v.apply(i as f32 / 1000.0));
        }
    }

    let start = Instant::now();

    for v in &variants {
        for i in 0u32..SAMPLES {
            // t sweeps the full [0,1] range repeatedly
            let t = (i % 1001) as f32 / 1000.0;
            black_box(v.apply(t));
        }
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_easing_all_variants: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({} easing calls). Bottleneck: Easing::Bounce / Elastic transcendentals.",
        variants.len() as u32 * SAMPLES
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. PARTICLE PHYSICS – SoA UPDATE LOOP
//    100 000 particles, 60 ticks at dt = 1/60.
//    Data stored in plain Vecs (SoA style) to keep it cache-friendly.
//    Limit: 300 ms — 6 000 000 Vec2 updates.  At ~4 ns/op = 24 ms; 300 ms
//    gives 12× headroom for debug builds.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_particle_soa_update() {
    if !stress_enabled() {
        return;
    }

    use glam::Vec2;
    const N: usize = 100_000;
    const TICKS: usize = 60;
    const LIMIT: Duration = Duration::from_millis(300);

    let gravity = Vec2::new(0.0, 98.0);
    let dt: f32 = 1.0 / 60.0;

    // Build SoA storage for maximum cache efficiency.
    // Separation of hot (position, velocity, lifetime) from cold (color, size)
    // matches the Data-Oriented Design goal of the engine.
    let mut positions: Vec<Vec2> = (0..N)
        .map(|i| Vec2::new((i % 800) as f32, (i % 600) as f32))
        .collect();
    let mut velocities: Vec<Vec2> = (0..N)
        .map(|i| Vec2::new((i % 200) as f32 - 100.0, (i % 150) as f32 - 75.0))
        .collect();
    let mut lifetimes: Vec<f32> = (0..N).map(|i| 1.0 + (i % 5) as f32).collect();

    // Warm-up: 5 ticks, not timed
    for _ in 0..5 {
        for ((pos, vel), lt) in positions
            .iter_mut()
            .zip(velocities.iter_mut())
            .zip(lifetimes.iter_mut())
        {
            *vel += gravity * dt;
            *pos += *vel * dt;
            *lt -= dt;
        }
    }

    // Reset lifetimes after warm-up
    for (i, lt) in lifetimes.iter_mut().enumerate() {
        *lt = 1.0 + (i % 5) as f32;
    }

    let start = Instant::now();

    for _ in 0..TICKS {
        for ((pos, vel), lt) in positions
            .iter_mut()
            .zip(velocities.iter_mut())
            .zip(lifetimes.iter_mut())
        {
            // Mirrors Particle::update() logic; direct struct update would
            // be AoS (Array of Structs), hurting cache for large N.
            *vel += gravity * dt;
            *pos += *vel * dt;
            *lt -= dt;
        }
        // Prevent the optimizer from proving the loop is a no-op.
        black_box(positions[0]);
        black_box(lifetimes[0]);
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_particle_soa_update: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({N} particles × {TICKS} ticks). Cache locality regression suspected."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. VfxSystem EMITTER THROUGHPUT
//    Spawn 500 explosion emitters (50 particles each = 25 000 particles),
//    then drive 120 update ticks at dt=1/60.
//    Limit: 400 ms — retain() + Vec2 update for 3 000 000 particle-ticks.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_vfx_system_emitter_throughput() {
    if !stress_enabled() {
        return;
    }

    use glam::Vec2;
    use rustgames::graphics::effects::system::VfxSystem;
    use rustgames::graphics::effects::{EmitterConfig, VfxEffect};

    const EMITTERS: u32 = 500;
    const TICKS: usize = 120;
    const LIMIT: Duration = Duration::from_millis(400);

    // warm-up: 5 emitters × 10 ticks
    {
        let mut sys = VfxSystem::new();
        for _ in 0..5 {
            sys.push(VfxEffect::Emitter(EmitterConfig::explosion(Vec2::ZERO)));
        }
        for _ in 0..10 {
            sys.update(1.0 / 60.0);
        }
        black_box(sys.count());
    }

    let mut sys = VfxSystem::new();
    for i in 0..EMITTERS {
        let pos = Vec2::new((i % 800) as f32, (i % 600) as f32);
        sys.push(VfxEffect::Emitter(EmitterConfig::explosion(pos)));
    }

    let start = Instant::now();

    for _ in 0..TICKS {
        sys.update(1.0 / 60.0);
        black_box(sys.count());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_vfx_system_emitter_throughput: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({EMITTERS} emitters × {TICKS} ticks). Bottleneck: Vec::retain or particle Vec2 math."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. CAMERA MATRIX BUILD UNDER CONTINUOUS SHAKE
//    Build the view-projection matrix 200 000 times while shake is active.
//    Limit: 500 ms — glam Mat4 orthographic + multiply ≈ ~30–50 ns/call in
//    debug; 200 000 × 50 ns = 10 ms; 500 ms is extremely conservative.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_camera_matrix_under_shake() {
    if !stress_enabled() {
        return;
    }

    use rustgames::graphics::camera::Camera;

    const FRAMES: u32 = 200_000;
    const LIMIT: Duration = Duration::from_millis(500);
    const DT: f32 = 1.0 / 60.0;

    let mut cam = Camera::new(1920, 1080);
    // Keep trauma pinned at 1.0 throughout: worst-case shake path.
    cam.add_trauma(1.0);

    // warm-up: 1 000 frames
    for _ in 0..1_000 {
        cam.add_trauma(1.0);
        cam.update(DT);
        black_box(cam.build_view_projection_matrix());
    }

    let start = Instant::now();

    for _ in 0..FRAMES {
        // Replenish trauma so the shake path is always active.
        cam.add_trauma(1.0);
        cam.update(DT);
        black_box(cam.build_view_projection_matrix());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_camera_matrix_under_shake: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({FRAMES} frames). Bottleneck: Mat4::orthographic_rh_gl or TraumaShake::update RNG."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. TRAUMA SHAKE STANDALONE – 1 000 000 updates
//    Isolated from Camera to measure only the shake math (sin/cos + RNG).
//    Limit: 300 ms — at 200 ns/call × 1 000 000 = 200 ms.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_trauma_shake_one_million_updates() {
    if !stress_enabled() {
        return;
    }

    use rustgames::graphics::effects::TraumaShake;

    const ITERATIONS: u32 = 1_000_000;
    const LIMIT: Duration = Duration::from_millis(300);
    const DT: f32 = 1.0 / 60.0;

    let mut shake = TraumaShake::new(20.0, 0.5);
    shake.add_trauma(1.0);

    // warm-up
    for _ in 0..5_000 {
        shake.add_trauma(0.01);
        shake.update(DT);
        black_box(shake.offset());
    }

    let start = Instant::now();

    for _ in 0..ITERATIONS {
        // Constantly re-add trauma so the hot path (sin/cos/rng) stays active.
        shake.add_trauma(0.001);
        shake.update(DT);
        black_box(shake.offset());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_trauma_shake_one_million_updates: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({ITERATIONS} updates). Bottleneck: sin/cos + rand::rng inside update()."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. TRANSLATION SYSTEM – O(1) LOOKUP UNDER 1 000 000 QUERIES
//    Pre-populate 10 000 entries, then hammer lookups.
//    Limit: 200 ms — FastHashMap O(1) amortised; keys are pre-hashed to u32
//    IDs before the timed region so no string hashing occurs in the hot loop.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_translation_lookup_one_million() {
    if !stress_enabled() {
        return;
    }

    use rustgames::translation::{TranslationSystem, generate_id_from_name};

    const ENTRIES: u32 = 10_000;
    const QUERIES: u32 = 1_000_000;
    const LIMIT: Duration = Duration::from_millis(200);

    // Pre-hash keys outside the timed region — the map stores (u32,u32) keys.
    let lang_id = generate_id_from_name("en");
    let text_ids: Vec<u32> = (0..ENTRIES)
        .map(|i| generate_id_from_name(&format!("key_{i}")))
        .collect();

    let mut sys = TranslationSystem::new();
    for i in 0..ENTRIES {
        sys.add_translation_by_name(&format!("key_{i}"), "en", format!("value_{i}"));
    }

    // warm-up
    for i in 0u32..10_000 {
        black_box(sys.get_translation(text_ids[(i % ENTRIES) as usize], lang_id));
    }

    let start = Instant::now();

    for i in 0u32..QUERIES {
        // O(1) FastHashMap lookup — no string allocation or hashing in hot path.
        black_box(sys.get_translation(text_ids[(i % ENTRIES) as usize], lang_id));
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_translation_lookup_one_million: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({QUERIES} lookups, {ENTRIES} entries). Possible FastHashMap regression."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. FpsCounter – 2 000 000 rolling-window updates
//    VecDeque push/pop pair per call; tests memory allocation churn and
//    summing logic.
//    Limit: 250 ms — 2M × ~100 ns (VecDeque op + f32 add) = 200 ms.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_fps_counter_two_million_updates() {
    if !stress_enabled() {
        return;
    }

    use rustgames::core::FpsCounter;

    const ITERATIONS: u32 = 2_000_000;
    const LIMIT: Duration = Duration::from_millis(250);

    let mut fps = FpsCounter::new();

    // warm-up: 10 000 frames with a realistic 60 Hz dt
    for i in 0u32..10_000 {
        let dt = 1.0 / 60.0 + (i % 5) as f32 * 0.0001;
        fps.update(dt);
        black_box(fps.fps());
    }

    let start = Instant::now();

    for i in 0u32..ITERATIONS {
        // Vary dt slightly to defeat constant-folding.
        let dt = 1.0 / 60.0 + (i % 5) as f32 * 0.0001;
        fps.update(dt);
        // Read all three aggregates to exercise the full hot path.
        black_box(fps.fps());
        black_box(fps.min_fps());
        black_box(fps.max_fps());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_fps_counter_two_million_updates: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({ITERATIONS} updates). Bottleneck: VecDeque churn or f32 total_time accumulation."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 9. ANIMATION SYSTEM – 10 000 concurrent animations × 1 000 ticks
//    Start 10 000 short-duration animations, drive them to completion.
//    Covers AnimationSystem::update + retain + easing evaluation.
//    Limit: 600 ms — 10M update calls at ~50 ns each = 500 ms.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_animation_system_massive_concurrent() {
    if !stress_enabled() {
        return;
    }

    use rustgames::graphics::Animation;
    use rustgames::graphics::effects::animation::animation_system::AnimationSystem;
    use rustgames::graphics::effects::animation::easing::Easing;

    const ANIMATIONS: usize = 10_000;
    const TICKS: usize = 1_000;
    const LIMIT: Duration = Duration::from_millis(600);

    let mut sys = AnimationSystem::new();

    // warm-up: 100 animations × 50 ticks
    for _ in 0..100 {
        sys.start(Animation::FadeIn { duration: 0.5 }, Easing::EaseInOut, 0.0);
    }
    for _ in 0..50 {
        sys.update(1.0 / 60.0);
    }

    // reset
    let mut sys = AnimationSystem::new();
    for i in 0..ANIMATIONS {
        // Mix of short and medium animations to keep retain() busy.
        let duration = 0.1 + (i % 10) as f32 * 0.05;
        let easing = match i % 6 {
            0 => Easing::Linear,
            1 => Easing::EaseIn,
            2 => Easing::EaseOut,
            3 => Easing::EaseInOut,
            4 => Easing::Bounce,
            _ => Easing::Elastic,
        };
        let anim = match i % 4 {
            0 => Animation::FadeIn { duration },
            1 => Animation::FadeOut { duration },
            2 => Animation::Scale {
                from: 0.5,
                to: 2.0,
                duration,
            },
            _ => Animation::Rotate {
                from: 0.0,
                to: std::f32::consts::TAU,
                duration,
            },
        };
        sys.start(anim, easing, 0.0);
    }

    let start = Instant::now();

    for _ in 0..TICKS {
        sys.update(1.0 / 60.0);
        black_box(sys.is_playing());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_animation_system_massive_concurrent: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({ANIMATIONS} animations × {TICKS} ticks). \
         Bottleneck: Vec::retain O(n) sweep or easing transcendentals."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 10. COLOR HEX PARSE FIREHOSE
//     Parse 500 000 hex strings to exercise string slicing, radix parsing,
//     and the full Color::from_hex code path.
//     Limit: 400 ms — u8::from_str_radix is cheap but not SIMD; 500 000 ×
//     ~600 ns = 300 ms; 400 ms allows headroom for slow debug alloc.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_color_hex_parse_firehose() {
    if !stress_enabled() {
        return;
    }

    use rustgames::graphics::color::Color;

    const ITERATIONS: u32 = 500_000;
    const LIMIT: Duration = Duration::from_millis(400);

    // Pre-build hex strings to avoid timing string formatting.
    let hexes: Vec<String> = (0u32..ITERATIONS)
        .map(|i| format!("{:06X}", i % 0x00FF_FFFF))
        .collect();

    // warm-up
    for h in hexes.iter().take(5_000) {
        black_box(Color::from_hex(h));
    }

    let start = Instant::now();

    for h in &hexes {
        black_box(Color::from_hex(h));
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_color_hex_parse_firehose: {elapsed:?} exceeded limit of {LIMIT:?} \
         ({ITERATIONS} hex parses). Bottleneck: str::get + u8::from_str_radix hot path."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 11. SPRITE INSTANCE CONSTRUCTION THROUGHPUT
//     Build 1 000 000 SpriteInstances via SpriteInstance::simple().
//     Uses a single from_scale_rotation_translation() call instead of
//     3 separate Mat4 builds + 2 multiplies.
//     Limit: 1000 ms — generous headroom for debug mode.
//     Under --release typically ≪ 50 ms.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_render_sprite_instance_construction() {
    if !stress_enabled() {
        return;
    }

    use glam::Vec2;
    use rustgames::graphics::SpriteInstance;

    const INSTANCES: u32 = 1_000_000;
    const LIMIT: Duration = Duration::from_millis(1000);

    // warm-up
    for i in 0u32..5_000 {
        let pos = Vec2::new((i % 800) as f32, (i % 600) as f32);
        let size = Vec2::new(32.0 + (i % 64) as f32, 32.0 + (i % 64) as f32);
        black_box(SpriteInstance::simple(
            pos,
            size,
            (i % 360) as f32 * 0.01745,
            1.0,
        ));
    }

    let start = Instant::now();

    for i in 0u32..INSTANCES {
        let pos = Vec2::new((i % 1920) as f32, (i % 1080) as f32);
        let size = Vec2::new(16.0 + (i % 128) as f32, 16.0 + (i % 128) as f32);
        let rot = (i % 628) as f32 * 0.01;
        let opacity = (i % 256) as f32 / 255.0;
        black_box(SpriteInstance::simple(pos, size, rot, opacity));
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_render_sprite_instance_construction: {elapsed:?} exceeded {LIMIT:?} \
         ({INSTANCES} instances). Bottleneck: Mat4 translate/rotate/scale or color pack."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 12. RENDER BATCH ASSEMBLY – INSTANCE SORT + PACK
//     Simulate building a full render frame: 10 000 sprites across 16 texture
//     buckets, sort by z-depth, pack into a flat Vec<SpriteInstance>.
//     100 frames — bucket vecs are pre-allocated and cleared each frame
//     to eliminate per-frame allocation churn.
//     Limit: 400 ms — sort O(n log n) + flat copy in debug mode.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_render_batch_assembly() {
    if !stress_enabled() {
        return;
    }

    use glam::Vec2;
    use rustgames::graphics::SpriteInstance;

    const SPRITES: usize = 10_000;
    const BUCKETS: usize = 16;
    const FRAMES: u32 = 100;
    const LIMIT: Duration = Duration::from_millis(400);

    // Pre-build sprites outside the timed region.
    let sprites: Vec<(usize, f32, SpriteInstance)> = (0..SPRITES)
        .map(|i| {
            let bucket = i % BUCKETS;
            let z = (i % 1000) as f32;
            let inst = SpriteInstance::simple(
                Vec2::new((i % 1920) as f32, (i % 1080) as f32),
                Vec2::splat(32.0),
                0.0,
                1.0,
            );
            (bucket, z, inst)
        })
        .collect();

    // Pre-allocate buckets and flat output once — cleared each frame.
    let mut buckets: Vec<Vec<(f32, SpriteInstance)>> = (0..BUCKETS)
        .map(|_| Vec::with_capacity(SPRITES / BUCKETS + 1))
        .collect();
    let mut flat: Vec<SpriteInstance> = Vec::with_capacity(SPRITES);

    // warm-up: 5 frames
    for _ in 0..5 {
        for b in &mut buckets {
            b.clear();
        }
        for &(bucket, z, inst) in &sprites {
            buckets[bucket].push((z, inst));
        }
        for b in &mut buckets {
            b.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        }
        flat.clear();
        flat.extend(buckets.iter().flat_map(|b| b.iter().map(|&(_, i)| i)));
        black_box(flat.len());
    }

    let start = Instant::now();

    for _ in 0..FRAMES {
        for b in &mut buckets {
            b.clear();
        }
        for &(bucket, z, inst) in &sprites {
            buckets[bucket].push((z, inst));
        }
        for b in &mut buckets {
            b.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        }
        flat.clear();
        flat.extend(buckets.iter().flat_map(|b| b.iter().map(|&(_, i)| i)));
        black_box(flat.len());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_render_batch_assembly: {elapsed:?} exceeded {LIMIT:?} \
         ({SPRITES} sprites × {FRAMES} frames, {BUCKETS} buckets). \
         Bottleneck: sort_unstable_by or flat extend."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 13. VIEW-PROJECTION MATRIX PIPELINE – FULL FRAME SIMULATION
//     Simulate the per-frame render transform: camera VP × per-sprite model.
//     500 000 sprite MVP matrices from a moving camera.
//     Limit: 400 ms — Mat4 multiply ≈ 16 mul+add each; ~400 ns/op × 500 000.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_render_mvp_matrix_pipeline() {
    if !stress_enabled() {
        return;
    }

    use glam::{Mat4, Vec3};
    use rustgames::graphics::camera::Camera;

    const SPRITES: u32 = 500_000;
    const LIMIT: Duration = Duration::from_millis(400);
    const DT: f32 = 1.0 / 60.0;

    let mut cam = Camera::new(1920, 1080);
    cam.update(DT);
    let vp = cam.build_view_projection_matrix();

    // Pre-build model matrices.
    let models: Vec<Mat4> = (0..SPRITES)
        .map(|i| {
            let t = Mat4::from_translation(Vec3::new((i % 1920) as f32, (i % 1080) as f32, 0.0));
            let r = Mat4::from_rotation_z((i % 628) as f32 * 0.01);
            let s = Mat4::from_scale(Vec3::new(32.0, 32.0, 1.0));
            t * r * s
        })
        .collect();

    // warm-up
    for m in models.iter().take(5_000) {
        black_box(vp * *m);
    }

    let start = Instant::now();

    for m in &models {
        // Full MVP = VP × Model — this is the per-sprite GPU upload cost.
        black_box(vp * *m);
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_render_mvp_matrix_pipeline: {elapsed:?} exceeded {LIMIT:?} \
         ({SPRITES} MVP multiplies). Bottleneck: glam Mat4 multiply."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 14. COLOR BLEND PIPELINE – ALPHA COMPOSITING THROUGHPUT
//     1 000 000 alpha-over composites (simulates per-particle blend in VFX
//     and transition overlay rendering).
//     Limit: 200 ms — pure f32 lerp × 4 channels + Color::new overhead in
//     debug mode; --release is typically ≪ 10 ms.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_render_alpha_composite_throughput() {
    if !stress_enabled() {
        return;
    }

    use rustgames::graphics::color::Color;

    const ITERATIONS: u32 = 1_000_000;
    const LIMIT: Duration = Duration::from_millis(200);

    // Simulate layered sprite color blending: tint × alpha-over background.
    let bg = Color::new(0.1, 0.2, 0.3, 1.0);

    // warm-up
    for i in 0u32..5_000 {
        let a = (i % 256) as f32 / 255.0;
        let tint = Color::new(
            (i % 256) as f32 / 255.0,
            ((i * 3) % 256) as f32 / 255.0,
            ((i * 7) % 256) as f32 / 255.0,
            a,
        );
        black_box(bg.lerp(tint, tint.a));
    }

    let start = Instant::now();

    for i in 0u32..ITERATIONS {
        let a = (i % 256) as f32 / 255.0;
        let tint = Color::new(
            (i % 256) as f32 / 255.0,
            ((i * 3) % 256) as f32 / 255.0,
            ((i * 7) % 256) as f32 / 255.0,
            a,
        );
        // Alpha-over: dst = lerp(bg, tint, tint.a)
        let blended = black_box(bg.lerp(tint, tint.a));
        black_box(blended.to_u32());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_render_alpha_composite_throughput: {elapsed:?} exceeded {LIMIT:?} \
         ({ITERATIONS} composites). Bottleneck: Color::lerp or to_u32 hot path."
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// 15. INSTANCE BUFFER BYTEMUCK CAST THROUGHPUT
//     Simulate what prepare_batch() does: cast 50 000 SpriteInstances to
//     raw bytes via bytemuck::cast_slice() — 50 000 × 96 bytes = 4.8 MB.
//     Repeat 200 frames.
//     Limit: 200 ms — memcpy bandwidth: 4.8 MB × 200 = 960 MB at ≫ 10 GB/s.
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn stress_render_instance_buffer_cast() {
    if !stress_enabled() {
        return;
    }

    use glam::Vec2;
    use rustgames::graphics::SpriteInstance;

    const INSTANCES: usize = 50_000;
    const FRAMES: u32 = 200;
    const LIMIT: Duration = Duration::from_millis(200);

    // Build instance buffer once — we re-cast it every frame.
    let instances: Vec<SpriteInstance> = (0..INSTANCES)
        .map(|i| {
            SpriteInstance::simple(
                Vec2::new((i % 1920) as f32, (i % 1080) as f32),
                Vec2::splat(32.0),
                (i % 628) as f32 * 0.01,
                1.0,
            )
        })
        .collect();

    // warm-up: 5 frames
    for _ in 0..5 {
        let bytes: &[u8] = bytemuck::cast_slice(&instances);
        black_box(bytes.len());
    }

    let start = Instant::now();

    for _ in 0..FRAMES {
        // This is exactly what SpriteRenderer::prepare_batch() does before
        // queue.write_buffer(). Zero-copy cast — measures cache/memory BW.
        let bytes: &[u8] = bytemuck::cast_slice(&instances);
        black_box(bytes.len());
    }

    let elapsed = start.elapsed();
    assert!(
        elapsed <= LIMIT,
        "stress_render_instance_buffer_cast: {elapsed:?} exceeded {LIMIT:?} \
         ({INSTANCES} instances × {FRAMES} frames = {} MB total). \
         Bottleneck: cache pressure or memory bandwidth.",
        INSTANCES * std::mem::size_of::<SpriteInstance>() * FRAMES as usize / (1024 * 1024)
    );
}
