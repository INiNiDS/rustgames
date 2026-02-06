pub mod prelude;
pub mod core;
pub mod graphics;
pub mod text;
pub mod window;
pub mod controllers;
pub mod audio;

#[cfg(test)]
mod tests {
    use crate::text::{RichTextParser, TextWrapper, FontWeight, TextStyle, TextAlignment, VerticalAlignment};
    use crate::graphics::color::Color;
    use crate::graphics::effects::effects::{EffectManager, EffectInstance, Particle, ParticleEffect, VisualEffect};
    use crate::graphics::effects::animation::animation::{Animation, AnimationGroupID};
    use crate::graphics::effects::animation::direction::Direction;
    use crate::graphics::effects::animation::easing::Easing;
    use crate::graphics::effects::animation::visual::{AnimEffect, VisualState, CombinedMode, CustomCombinedMode};
    use crate::graphics::effects::animation::animation_instance::AnimationInstance;
    use crate::graphics::effects::animation::timeline::TimelineBuilder;
    use crate::graphics::effects::shake_effect::TraumaShake;
    use crate::graphics::sprite_animation::{SpriteAnimation, AnimationMode};
    use crate::graphics::render::instance::SpriteInstance;
    use crate::controllers::animation_controller::AnimationController;
    use crate::core::fps_counter::FpsCounter;
    use crate::window::{Event, EventQueue, WindowConfig};
    use crate::text::typewriter::{TypewriterEffect, TypewriterInstance, TextSpeed};
    use crate::graphics::render::renderer_alpha::RendererAlpha;
    use glam::{Vec2, Vec4};

    #[test]
    fn test_text_wrapping() {
        let text = "Hello world this is a test";
        let lines = TextWrapper::wrap_text(text, 50.0, 10.0);
        assert!(!lines.is_empty());
    }

    #[test]
    fn test_rich_text_parsing() {
        let segments = RichTextParser::parse("Hello [b]world[/b]!");
        assert_eq!(segments.len(), 3);
    }

    #[test]
    fn test_text_wrapping_single_word() {
        let lines = TextWrapper::wrap_text("Hello", 100.0, 10.0);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Hello");
    }

    #[test]
    fn test_text_wrapping_empty_string() {
        let lines = TextWrapper::wrap_text("", 100.0, 10.0);
        assert!(lines.is_empty());
    }

    #[test]
    fn test_text_wrapping_forces_line_break() {
        let lines = TextWrapper::wrap_text("aaa bbb ccc", 35.0, 10.0);
        assert!(lines.len() >= 2);
    }

    #[test]
    fn test_rich_text_bold_weight() {
        let segments = RichTextParser::parse("[b]bold[/b]");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].attrs.weight, FontWeight::Bold);
    }

    #[test]
    fn test_rich_text_italic() {
        let segments = RichTextParser::parse("[i]italic[/i]");
        assert_eq!(segments.len(), 1);
        assert!(segments[0].attrs.italic);
    }

    #[test]
    fn test_rich_text_nested() {
        let segments = RichTextParser::parse("[b]bold [i]bolditalic[/i][/b]");
        assert!(segments.len() >= 2);
    }

    #[test]
    fn test_rich_text_color() {
        let segments = RichTextParser::parse("[color=#FF0000]red[/color]");
        assert_eq!(segments.len(), 1);
        assert!(segments[0].attrs.color.is_some());
    }

    #[test]
    fn test_rich_text_plain_text_only() {
        let segments = RichTextParser::parse("just plain text");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].attrs.weight, FontWeight::Normal);
        assert!(!segments[0].attrs.italic);
    }

    #[test]
    fn test_color_from_hex_6_digits() {
        let color = Color::from_hex("#FF8800").unwrap();
        assert!((color.r - 1.0).abs() < 0.01);
        assert!((color.g - 0.533).abs() < 0.01);
        assert!(color.b.abs() < 0.01);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_color_from_hex_8_digits() {
        let color = Color::from_hex("#FF880080").unwrap();
        assert!((color.a - 0.502).abs() < 0.01);
    }

    #[test]
    fn test_color_from_hex_invalid() {
        assert!(Color::from_hex("#ZZZ").is_none());
        assert!(Color::from_hex("#12345").is_none());
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::WHITE.to_array(), [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(Color::BLACK.to_array(), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(Color::TRANSPARENT.to_array(), [0.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_color_lerp() {
        let result = Color::BLACK.lerp(Color::WHITE, 0.5);
        assert!((result.r - 0.5).abs() < 0.01);
        assert!((result.g - 0.5).abs() < 0.01);
        assert!((result.b - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_color_lerp_clamped() {
        let result = Color::BLACK.lerp(Color::WHITE, 2.0);
        assert_eq!(result.r, 1.0);
    }

    #[test]
    fn test_color_with_alpha() {
        let color = Color::RED.with_alpha(0.5);
        assert_eq!(color.r, 1.0);
        assert_eq!(color.a, 0.5);
    }

    #[test]
    fn test_color_to_u32() {
        let val = Color::WHITE.to_u32();
        assert_eq!(val, 0xFFFFFFFF);
    }

    #[test]
    fn test_color_from_rgba_u8() {
        let color = Color::from_rgba_u8(255, 128, 0, 255);
        assert!((color.r - 1.0).abs() < 0.01);
        assert!((color.g - 0.502).abs() < 0.01);
        assert!(color.b.abs() < 0.01);
    }

    #[test]
    fn test_color_from_tuple() {
        let color: Color = (0.5, 0.6, 0.7).into();
        assert!((color.r - 0.5).abs() < f32::EPSILON);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_easing_linear() {
        assert_eq!(Easing::Linear.apply(0.0), 0.0);
        assert_eq!(Easing::Linear.apply(0.5), 0.5);
        assert_eq!(Easing::Linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_easing_ease_in() {
        let val = Easing::EaseIn.apply(0.5);
        assert!(val < 0.5);
    }

    #[test]
    fn test_easing_ease_out() {
        let val = Easing::EaseOut.apply(0.5);
        assert!(val > 0.5);
    }

    #[test]
    fn test_easing_clamped() {
        assert_eq!(Easing::Linear.apply(-1.0), 0.0);
        assert_eq!(Easing::Linear.apply(2.0), 1.0);
    }

    #[test]
    fn test_easing_bounce_at_one() {
        let val = Easing::Bounce.apply(1.0);
        assert!((val - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_easing_elastic_at_boundaries() {
        assert_eq!(Easing::Elastic.apply(0.0), 0.0);
        assert_eq!(Easing::Elastic.apply(1.0), 1.0);
    }

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::Left.opposite(), Direction::Right);
        assert_eq!(Direction::Top.opposite(), Direction::Bottom);
        assert_eq!(Direction::TopLeft.opposite(), Direction::BottomRight);
    }

    #[test]
    fn test_direction_to_vector_normalized() {
        let v = Direction::TopLeft.to_vector();
        let len = (v.x * v.x + v.y * v.y).sqrt();
        assert!((len - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_visual_state_default() {
        let state = VisualState::default();
        assert_eq!(state.opacity, 1.0);
        assert_eq!(state.position, Vec2::ZERO);
        assert_eq!(state.scale, Vec2::ONE);
        assert_eq!(state.rotation, 0.0);
    }

    #[test]
    fn test_anim_effect_combine() {
        let a = AnimEffect::with_opacity(0.5);
        let b = AnimEffect::with_opacity(0.5);
        let combined = a.combine(b);
        assert!((combined.opacity_mul - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_anim_effect_apply_default() {
        let effect = AnimEffect::with_offset(Vec2::new(10.0, 20.0));
        let state = VisualState::default();
        let result = effect.apply_to_default(state);
        assert_eq!(result.position, Vec2::new(10.0, 20.0));
        assert_eq!(result.opacity, 1.0);
    }

    #[test]
    fn test_anim_effect_apply_override_mode() {
        let effect = AnimEffect::with_scale(Vec2::new(2.0, 3.0));
        let state = VisualState::default();
        let config = CustomCombinedMode::with_scale(CombinedMode::Override);
        let result = effect.apply_to_config(state, config);
        assert_eq!(result.scale, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_animation_instance_progress() {
        let inst = AnimationInstance::new(0, Animation::FadeIn { duration: 2.0 }, Easing::Linear, 0.0);
        assert_eq!(inst.progress(), 0.0);
    }

    #[test]
    fn test_animation_instance_update_and_finish() {
        let mut inst = AnimationInstance::new(0, Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        inst.update(1.0);
        assert!(inst.is_finished());
    }

    #[test]
    fn test_animation_instance_delay() {
        let mut inst = AnimationInstance::new(0, Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.5);
        inst.update(0.3);
        assert_eq!(inst.progress(), 0.0);
        inst.update(0.3);
        assert!(inst.progress() > 0.0);
    }

    #[test]
    fn test_animation_instance_pause() {
        let mut inst = AnimationInstance::new(0, Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        inst.paused = true;
        inst.update(0.5);
        assert_eq!(inst.progress(), 0.0);
    }

    #[test]
    fn test_animation_controller_start_stop() {
        let mut controller = AnimationController::new();
        let id = controller.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        assert!(controller.is_playing_id(id));
        controller.stop(id);
        assert!(!controller.is_playing_id(id));
    }

    #[test]
    fn test_animation_controller_auto_remove() {
        let mut controller = AnimationController::new();
        controller.start(Animation::FadeIn { duration: 0.5 }, Easing::Linear, 0.0);
        assert_eq!(controller.count(), 1);
        controller.update(1.0);
        assert_eq!(controller.count(), 0);
    }

    #[test]
    fn test_animation_controller_sequence() {
        let mut controller = AnimationController::new();
        let group = controller.start_sequence(vec![
            (Animation::FadeIn { duration: 0.5 }, Easing::Linear),
            (Animation::FadeOut { duration: 0.5 }, Easing::Linear),
        ]);
        assert_eq!(group.len(), 2);
        assert!(!group.is_empty());
    }

    #[test]
    fn test_animation_controller_parallel() {
        let mut controller = AnimationController::new();
        let group = controller.start_parallel(vec![
            (Animation::FadeIn { duration: 1.0 }, Easing::Linear),
            (Animation::Scale { from: 0.0, to: 1.0, duration: 1.0 }, Easing::EaseOut),
        ]);
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn test_animation_controller_clear() {
        let mut controller = AnimationController::new();
        controller.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        controller.start(Animation::FadeOut { duration: 1.0 }, Easing::Linear, 0.0);
        controller.clear();
        assert_eq!(controller.count(), 0);
    }

    #[test]
    fn test_animation_controller_evaluate() {
        let mut controller = AnimationController::new();
        controller.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        controller.update(0.5);
        let state = controller.evaluate(VisualState::default(), Vec2::new(100.0, 100.0), None);
        assert!(state.opacity < 1.0);
    }

    #[test]
    fn test_timeline_builder() {
        let steps = TimelineBuilder::new()
            .single(Animation::FadeIn { duration: 0.5 }, Easing::Linear)
            .gap(0.2)
            .single(Animation::FadeOut { duration: 0.5 }, Easing::Linear)
            .build();
        assert_eq!(steps.len(), 3);
    }

    #[test]
    fn test_effect_manager_lifecycle() {
        let mut mgr = EffectManager::new();
        mgr.add_effect(VisualEffect::Flash { color: Color::WHITE, duration: 0.5 });
        assert_eq!(mgr.count(), 1);
        mgr.update(1.0);
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_effect_manager_clear() {
        let mut mgr = EffectManager::new();
        mgr.add_effect(VisualEffect::Vignette { intensity: 0.5 });
        mgr.clear();
        assert_eq!(mgr.count(), 0);
    }

    #[test]
    fn test_particle_effect_presets_particle_count() {
        let sparkles = ParticleEffect::sparkles(Vec2::ZERO);
        assert_eq!(sparkles.particle_count, 20);
        let explosion = ParticleEffect::explosion(Vec2::ZERO);
        assert_eq!(explosion.particle_count, 50);
        let rain = ParticleEffect::rain(Vec2::ZERO);
        assert_eq!(rain.particle_count, 200);
    }

    #[test]
    fn test_particle_update_moves_position() {
        let mut p = Particle::new(Vec2::ZERO, Vec2::new(10.0, 0.0), 1.0, Color::WHITE, 5.0);
        p.update(0.1, Vec2::ZERO);
        assert!(p.position.x > 0.0);
    }

    #[test]
    fn test_particle_lifetime() {
        let mut p = Particle::new(Vec2::ZERO, Vec2::ZERO, 0.5, Color::WHITE, 5.0);
        assert!(p.is_alive());
        p.update(0.6, Vec2::ZERO);
        assert!(!p.is_alive());
    }

    #[test]
    fn test_effect_instance_flash_duration() {
        let inst = EffectInstance::new(VisualEffect::Flash { color: Color::RED, duration: 0.3 });
        assert!((inst.duration() - 0.3).abs() < f32::EPSILON);
    }

    #[test]
    fn test_effect_instance_vignette_infinite() {
        let inst = EffectInstance::new(VisualEffect::Vignette { intensity: 0.5 });
        assert!(inst.duration().is_infinite());
        assert!(!inst.is_complete());
    }

    #[test]
    fn test_trauma_shake_basics() {
        let mut shake = TraumaShake::new(10.0, 2.0);
        assert!(!shake.is_active());
        shake.add_trauma(0.5);
        assert!(shake.is_active());
        assert_eq!(shake.trauma(), 0.5);
    }

    #[test]
    fn test_trauma_shake_offset_nonzero_when_active() {
        let mut shake = TraumaShake::new(10.0, 1.0);
        shake.add_trauma(0.8);
        shake.update(0.016);
        let offset = shake.offset();
        let magnitude = (offset.x * offset.x + offset.y * offset.y).sqrt();
        assert!(magnitude > 0.0);
    }

    #[test]
    fn test_sprite_instance_simple_defaults() {
        let inst = SpriteInstance::simple(Vec2::ZERO, Vec2::new(64.0, 64.0), 0.0, 1.0);
        assert_eq!(inst.uv_rect, [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(inst.color[3], 1.0);
    }

    #[test]
    fn test_sprite_instance_custom_color() {
        let inst = SpriteInstance::new(
            Vec2::ZERO,
            Vec2::ONE,
            0.0,
            Vec4::new(0.0, 0.0, 1.0, 1.0),
            Vec4::new(1.0, 0.0, 0.0, 0.5),
        );
        assert_eq!(inst.color, [1.0, 0.0, 0.0, 0.5]);
    }

    #[test]
    fn test_fps_counter_empty() {
        let counter = FpsCounter::new();
        assert_eq!(counter.fps(), 0.0);
        assert_eq!(counter.frame_time_ms(), 0.0);
    }

    #[test]
    fn test_event_queue_key_tracking() {
        let mut queue = EventQueue::new();
        queue.push(Event::KeyPressed(winit::keyboard::KeyCode::KeyW));
        assert!(queue.is_key_pressed(winit::keyboard::KeyCode::KeyW));
        assert!(queue.was_key_just_pressed(winit::keyboard::KeyCode::KeyW));
        queue.push(Event::KeyReleased(winit::keyboard::KeyCode::KeyW));
        assert!(!queue.is_key_pressed(winit::keyboard::KeyCode::KeyW));
    }

    #[test]
    fn test_event_queue_drain() {
        let mut queue = EventQueue::new();
        queue.push(Event::WindowClosed);
        queue.push(Event::WindowFocused(true));
        let events = queue.drain();
        assert_eq!(events.len(), 2);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_window_config_default() {
        let config = WindowConfig::default();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 600);
        assert!(config.resizable);
        assert!(config.vsync);
    }

    #[test]
    fn test_window_config_new() {
        let config = WindowConfig::new("My Game", 1920, 1080);
        assert_eq!(config.title, "My Game");
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
    }

    #[test]
    fn test_typewriter_effect_basic() {
        let mut tw = TypewriterEffect::new("Hello", TextSpeed::Fast, 0, 0.0, 0.0);
        assert!(!tw.is_complete());
        assert_eq!(tw.visible_text(), "");
        tw.update(1.0);
        assert!(tw.visible_text().len() > 0);
    }

    #[test]
    fn test_typewriter_effect_instant() {
        let tw = TypewriterEffect::new("Hello", TextSpeed::Instant, 0, 0.0, 0.0);
        assert!(tw.is_complete());
        assert_eq!(tw.visible_text(), "Hello");
    }

    #[test]
    fn test_typewriter_effect_skip() {
        let mut tw = TypewriterEffect::new("Hello World", TextSpeed::Slow, 0, 0.0, 0.0);
        tw.skip();
        assert!(tw.is_complete());
        assert_eq!(tw.visible_text(), "Hello World");
    }

    #[test]
    fn test_typewriter_effect_reset() {
        let mut tw = TypewriterEffect::new("Hello", TextSpeed::Fast, 0, 0.0, 0.0);
        tw.skip();
        tw.reset();
        assert!(!tw.is_complete());
        assert_eq!(tw.visible_text(), "");
    }

    #[test]
    fn test_typewriter_effect_progress() {
        let tw = TypewriterEffect::new("Hello", TextSpeed::Instant, 0, 0.0, 0.0);
        assert_eq!(tw.progress(), 1.0);
    }

    #[test]
    fn test_typewriter_instance_add_remove() {
        let mut inst = TypewriterInstance::new();
        let id = inst.add_typewriter_effect("Test", TextSpeed::Fast, 0.0, 0.0);
        assert_eq!(inst.len(), 1);
        inst.remove_typewriter_effect(id);
        assert!(inst.is_empty());
    }

    #[test]
    fn test_typewriter_instance_get() {
        let mut inst = TypewriterInstance::new();
        let id = inst.add_typewriter_effect("Hello", TextSpeed::Instant, 10.0, 20.0);
        let effect = inst.get_effect(id).unwrap();
        assert_eq!(effect.full_text(), "Hello");
        assert_eq!(effect.x, 10.0);
        assert_eq!(effect.y, 20.0);
    }

    #[test]
    fn test_text_speed_values() {
        assert_eq!(TextSpeed::Slow.chars_per_second(), 20.0);
        assert_eq!(TextSpeed::Medium.chars_per_second(), 40.0);
        assert_eq!(TextSpeed::Fast.chars_per_second(), 80.0);
        assert!(TextSpeed::Instant.chars_per_second().is_infinite());
        assert_eq!(TextSpeed::Custom(100.0).chars_per_second(), 100.0);
    }

    #[test]
    fn test_text_style_default() {
        let style = TextStyle::default();
        assert_eq!(style.size, 16.0);
        assert_eq!(style.alignment, TextAlignment::Left);
        assert_eq!(style.vertical_alignment, VerticalAlignment::Top);
    }

    #[test]
    fn test_text_style_builder() {
        let style = TextStyle::new(32.0)
            .with_color(Color::RED)
            .with_alignment(TextAlignment::Center);
        assert_eq!(style.size, 32.0);
        assert_eq!(style.alignment, TextAlignment::Center);
    }

    #[test]
    fn test_text_measure() {
        let (w, h) = TextWrapper::measure_text("Hello", 16.0);
        assert!(w > 0.0);
        assert!(h > 0.0);
    }

    #[test]
    fn test_animation_group_id_empty() {
        let group = AnimationGroupID::empty();
        assert!(group.is_empty());
        assert_eq!(group.len(), 0);
    }

    #[test]
    fn test_renderer_alpha_multiple_effects() {
        let mut ra = RendererAlpha::new();
        ra.add_effect(VisualEffect::Flash { color: Color::WHITE, duration: 1.0 });
        ra.add_effect(VisualEffect::ScreenShake { intensity: 5.0, duration: 0.5 });
        ra.add_effect(VisualEffect::Particles(ParticleEffect::new(Vec2::ZERO)));
        assert_eq!(ra.active_effect_count(), 3);
        ra.update(0.1);
        let frame = ra.build_effect_frame();
        assert!(frame.flash_color.is_some());
        assert_ne!(frame.screen_shake_offset, Vec2::ZERO);
    }

    #[test]
    fn test_animation_controller_pause_resume() {
        let mut controller = AnimationController::new();
        let id = controller.start(Animation::FadeIn { duration: 2.0 }, Easing::Linear, 0.0);
        assert!(controller.pause(id));
        controller.update(1.0);
        assert!(controller.is_playing_id(id));
        assert!(controller.resume(id));
        controller.update(3.0);
        assert!(!controller.is_playing_id(id));
    }

    #[test]
    fn test_animation_controller_replace() {
        let mut controller = AnimationController::new();
        let id = controller.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        controller.update(0.5);
        assert!(controller.replace(id, Animation::FadeOut { duration: 2.0 }));
        assert!(controller.is_playing_id(id));
    }

    #[test]
    fn test_color_partial_eq_ignores_alpha() {
        let a = Color::new(1.0, 0.0, 0.0, 1.0);
        let b = Color::new(1.0, 0.0, 0.0, 0.5);
        assert_eq!(a, b);
    }

    #[test]
    fn test_sprite_animation_set_frame() {
        let frames = vec![
            Vec4::new(0.0, 0.0, 0.5, 0.5),
            Vec4::new(0.5, 0.0, 0.5, 0.5),
            Vec4::new(0.0, 0.5, 0.5, 0.5),
        ];
        let mut anim = SpriteAnimation::new(frames, 10.0, AnimationMode::Loop);
        anim.set_frame(2);
        assert_eq!(anim.current_frame_index(), 2);
        anim.set_frame(100);
        assert_eq!(anim.current_frame_index(), 2);
    }
}
