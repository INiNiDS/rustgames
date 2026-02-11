use crate::graphics::effects::{AnimationGroupID, CustomCombinedMode, TimelineStep};
use crate::prelude::{ActiveAnimation, AnimEffect, Animation, Easing, VisualState};
use glam::Vec2;

/// Manages animation instances: starting, stopping, pausing, seeking, and
/// evaluating combined effects on a `VisualState`.
pub struct AnimationSystem {
    pub(crate) animations: Vec<ActiveAnimation>,
    pub(crate) next_id: usize,
}

impl AnimationSystem {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            animations: Vec::new(),
            next_id: 0,
        }
    }

    pub fn start(&mut self, animation: Animation, easing: Easing, delay: f32) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.animations
            .push(ActiveAnimation::new(id, animation, easing, delay));
        id
    }

    pub fn stop(&mut self, id: usize) {
        self.animations.retain(|a| a.id != id);
    }

    pub fn update(&mut self, delta_time: f32) {
        for anim in &mut self.animations {
            anim.update(delta_time);
        }
        self.animations.retain(|anim| !anim.is_finished());
    }

    #[must_use]
    pub const fn is_playing(&self) -> bool {
        !self.animations.is_empty()
    }

    pub fn pause(&mut self, id: usize) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.paused = true;
            true
        } else {
            false
        }
    }

    pub fn resume(&mut self, id: usize) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.paused = false;
            true
        } else {
            false
        }
    }

    pub fn set_playback(&mut self, id: usize, speed: f32) -> bool {
        if speed == 0.0 {
            self.pause(id);
        }
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.playback = speed;
            true
        } else {
            false
        }
    }

    pub fn seek_time(&mut self, id: usize, time: f32) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.elapsed = time.clamp(0.0, anim.duration());
            true
        } else {
            false
        }
    }

    pub fn seek_progress(&mut self, id: usize, progress: f32) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.elapsed = progress * anim.duration();
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_playing_id(&self, id: usize) -> bool {
        self.animations.iter().any(|a| a.id == id)
    }

    #[must_use]
    pub const fn count(&self) -> usize {
        self.animations.len()
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }

    #[must_use]
    pub fn evaluate(
        &self,
        base: VisualState,
        size: Vec2,
        mode: Option<CustomCombinedMode>,
    ) -> VisualState {
        let mut combined = AnimEffect::default();
        for anim in &self.animations {
            combined = combined.combine(anim.effect(size));
        }
        combined.apply_to(base, mode)
    }

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

    pub const fn apply_animation(&mut self) {}

    pub(crate) fn spawn_instance(
        &mut self,
        anim: Animation,
        easing: Easing,
        delay: f32,
    ) -> (usize, f32) {
        let id = self.next_id;
        self.next_id += 1;
        let inst = ActiveAnimation::new(id, anim, easing, delay);
        let dur = inst.duration();
        self.animations.push(inst);
        (id, dur)
    }
}

impl Default for AnimationSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
