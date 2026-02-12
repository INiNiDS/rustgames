use super::animation_system::AnimationSystem;
use crate::graphics::effects::{AnimationGroupID, TimelineStep};
use crate::prelude::Animation;

/// Utility / convenience methods for `AnimationSystem`.
impl AnimationSystem {
    pub fn start_timeline(&mut self, steps: Vec<TimelineStep>) -> AnimationGroupID {
        let mut ids = Vec::new();
        let mut delay_acc = 0.0;

        for step in steps {
            match step {
                TimelineStep::Gap(t) => delay_acc += t.max(0.0),
                TimelineStep::Single(anim, easing) => {
                    let (id, dur) = self.spawn_instance(anim, easing, delay_acc);
                    delay_acc += dur;
                    ids.push(id);
                }
                TimelineStep::Parallel(anims) => {
                    let mut max_len: f32 = 0.0;
                    for (anim, easing) in anims {
                        let (id, dur) = self.spawn_instance(anim, easing, delay_acc);
                        max_len = max_len.max(dur);
                        ids.push(id);
                    }
                    delay_acc += max_len;
                }
            }
        }
        AnimationGroupID::new(ids)
    }

    pub fn stop_all(&mut self) {
        for anim in &mut self.animations {
            anim.stop();
        }
    }

    pub fn stop_by_predicate(&mut self, predicate: fn(&Animation) -> bool) {
        self.animations.retain(|anim| !predicate(&anim.animation));
    }

    pub fn replace(&mut self, id: usize, animation: Animation) -> bool {
        if let Some(inst) = self.animations.iter_mut().find(|a| a.id == id) {
            inst.animation = animation;
            inst.elapsed = 0.0;
            true
        } else {
            false
        }
    }

    pub fn restart(&mut self, id: usize) {
        self.seek_progress(id, 0.0);
    }

    pub fn set_delay(&mut self, delay: f32, ids: Vec<usize>) {
        let delay = delay.max(0.0);
        for id in ids {
            if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
                anim.set_delay(delay);
            }
        }
    }

    pub fn add_delay(&mut self, delay: f32, ids: Vec<usize>) {
        let delay = delay.max(0.0);
        for id in ids {
            if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
                anim.add_delay(delay);
            }
        }
    }

    #[allow(clippy::unused_self)]
    pub const fn apply_animation(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::graphics::effects::animation::animation_system::AnimationSystem;
    use crate::prelude::{Animation, Easing, VisualState};
    use glam::Vec2;

    #[test]
    fn start_stop() {
        let mut sys = AnimationSystem::new();
        let id = sys.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        assert!(sys.is_playing_id(id));
        sys.stop(id);
        assert!(!sys.is_playing_id(id));
    }

    #[test]
    fn auto_removes_finished() {
        let mut sys = AnimationSystem::new();
        sys.start(Animation::FadeIn { duration: 0.5 }, Easing::Linear, 0.0);
        assert_eq!(sys.count(), 1);
        sys.update(1.0);
        assert_eq!(sys.count(), 0);
    }

    #[test]
    fn sequence_creates_group() {
        let mut sys = AnimationSystem::new();
        let group = sys.start_sequence(vec![
            (Animation::FadeIn { duration: 0.5 }, Easing::Linear),
            (Animation::FadeOut { duration: 0.5 }, Easing::Linear),
        ]);
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn parallel_creates_group() {
        let mut sys = AnimationSystem::new();
        let group = sys.start_parallel(vec![
            (Animation::FadeIn { duration: 1.0 }, Easing::Linear),
            (
                Animation::Scale {
                    from: 0.0,
                    to: 1.0,
                    duration: 1.0,
                },
                Easing::EaseOut,
            ),
        ]);
        assert_eq!(group.len(), 2);
    }

    #[test]
    fn clear_removes_all() {
        let mut sys = AnimationSystem::new();
        sys.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        sys.start(Animation::FadeOut { duration: 1.0 }, Easing::Linear, 0.0);
        sys.clear();
        assert_eq!(sys.count(), 0);
    }

    #[test]
    fn evaluate_opacity() {
        let mut sys = AnimationSystem::new();
        sys.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        sys.update(0.5);
        let state = sys.evaluate(VisualState::default(), Vec2::new(100.0, 100.0), None);
        assert!(state.opacity < 1.0);
    }

    #[test]
    fn pause_resume() {
        let mut sys = AnimationSystem::new();
        let id = sys.start(Animation::FadeIn { duration: 2.0 }, Easing::Linear, 0.0);
        assert!(sys.pause(id));
        sys.update(1.0);
        assert!(sys.is_playing_id(id));
        assert!(sys.resume(id));
        sys.update(3.0);
        assert!(!sys.is_playing_id(id));
    }

    #[test]
    fn replace_resets_animation() {
        let mut sys = AnimationSystem::new();
        let id = sys.start(Animation::FadeIn { duration: 1.0 }, Easing::Linear, 0.0);
        sys.update(0.5);
        assert!(sys.replace(id, Animation::FadeOut { duration: 2.0 }));
        assert!(sys.is_playing_id(id));
    }
}
