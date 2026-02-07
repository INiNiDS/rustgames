use glam::Vec2;
use crate::graphics::effects::{TimelineStep, CustomCombinedMode, AnimationGroupID};
use crate::prelude::{AnimEffect, Animation, AnimationInstance, Easing, VisualState};

/// Manages animation instances: starting, stopping, pausing, seeking, and
/// evaluating combined effects on a `VisualState`.
pub struct AnimationController {
    animations: Vec<AnimationInstance>,
    next_id: usize,
}

impl AnimationController {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            next_id: 0,
        }
    }

    pub fn start(&mut self, animation: Animation, easing: Easing, delay: f32) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.animations.push(AnimationInstance::new(id, animation, easing, delay));
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

    pub fn is_playing(&self) -> bool {
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
        } else {false}
    }

    pub fn seek_time(&mut self, id: usize, time: f32) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.elapsed = time.clamp(0.0, anim.duration());
            true
        } else {false}
    }

    pub fn seek_progress(&mut self, id: usize, progress: f32) -> bool {
        if let Some(anim) = self.animations.iter_mut().find(|a| a.id == id) {
            anim.elapsed = progress * anim.duration();
            true
        } else {false}
    }

    pub fn is_playing_id(&self, id: usize) -> bool {
        self.animations.iter().any(|a| a.id == id)
    }

    pub fn count(&self) -> usize {
        self.animations.len()
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }

    pub fn evaluate(&self, base: VisualState, size: Vec2, custom_combined_mode: Option<CustomCombinedMode>) -> VisualState {
        let mut combined = AnimEffect::default();
        for anim in &self.animations {
            let effect = anim.effect(size);
            combined = combined.combine(effect);
        };
        combined.apply_to(base, custom_combined_mode)
    }

    pub fn start_sequence(
        &mut self,
        steps: Vec<(Animation, Easing)>
    ) -> AnimationGroupID {
        let mut ids = Vec::with_capacity(steps.len());
        let mut delay_acc = 0.0;

        for (anim, easing) in steps {
            let (id, dur) = self.spawn_instance(anim, easing, delay_acc);
            delay_acc += dur;
            ids.push(id);
        }

        AnimationGroupID::new(ids)
    }


    pub fn start_parallel(
        &mut self,
        steps: Vec<(Animation, Easing)>
    ) -> AnimationGroupID {
        let mut ids = Vec::with_capacity(steps.len());

        for (anim, easing) in steps {
            let (id, _) = self.spawn_instance(anim, easing, 0.0);
            ids.push(id);
        }

        AnimationGroupID::new(ids)
    }


    pub fn start_parallel_with_delay(
        &mut self,
        steps: Vec<(Animation, Easing)>,
        start_delay: f32,
    ) -> AnimationGroupID {
        let mut ids = Vec::with_capacity(steps.len());

        for (anim, easing) in steps {
            let (id, _) = self.spawn_instance(anim, easing, start_delay);
            ids.push(id);
        }

        AnimationGroupID::new(ids)
    }


    pub fn start_timeline(&mut self, steps: Vec<TimelineStep>) -> AnimationGroupID {
        let mut ids = Vec::new();
        let mut delay_acc = 0.0;

        for step in steps {
            match step {
                TimelineStep::Gap(t) => {
                    delay_acc += t.max(0.0);
                }

                TimelineStep::Single(anim, easing) => {
                    self.handle_single(anim, easing, &mut delay_acc, &mut ids);
                }

                TimelineStep::Parallel(anims) => {
                    self.handle_parallel(anims, &mut delay_acc, &mut ids);
                }
            }
        }

        AnimationGroupID::new(ids)
    }


    pub fn stop_by_group(&mut self, group: &AnimationGroupID) {
        for id in group.iter() {
            self.stop(*id);
        }
    }

    pub fn remove_from_group(&mut self, group: &mut AnimationGroupID, index: usize)  {
        if let Some(&id) = group.get_id(index) { self.stop(id); }
        group.remove(index);
    }

    pub fn is_group_playing_all(&self, group: &AnimationGroupID) -> bool {
        group.iter().all(|id| self.is_playing_id(*id))
    }

    pub fn is_group_finished_all(&self, group: &AnimationGroupID) -> bool {
        group.iter().all(|id| !self.is_playing_id(*id))
    }

    pub fn is_group_playing_any(&self, group: &AnimationGroupID) -> bool {
        group.iter().any(|id| self.is_playing_id(*id))
    }

    pub fn is_group_finished_any(&self, group: &AnimationGroupID) -> bool {
        group.iter().any(|id| !self.is_playing_id(*id))
    }

    pub fn pause_group(&mut self, group: &AnimationGroupID) {
        for id in group.iter() {
            self.pause(*id);
        }
    }

    pub fn resume_group(&mut self, group: &AnimationGroupID) {
        for id in group.iter() {
            self.resume(*id);
        }
    }

    pub fn stop_all(&mut self) {
        for anim in self.animations.iter_mut() {
            anim.stop()
        }
    }

    pub fn stop_by_predicate(&mut self, predicate: fn(&Animation) -> bool) {
        self.animations.retain(|anim| !predicate(&anim.animation))
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

    pub fn restart_group(&mut self, group: &AnimationGroupID) {
        for id in group.iter() {
            self.seek_progress(*id, 0.0);
        }
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

    pub fn apply_animation(&mut self) {}

    fn spawn_instance(
        &mut self,
        anim: Animation,
        easing: Easing,
        delay: f32,
    ) -> (usize, f32) {
        let id = self.next_id;
        self.next_id += 1;

        let inst = AnimationInstance::new(id, anim, easing, delay);
        let dur = inst.duration();

        self.animations.push(inst);
        (id, dur)
    }


    fn handle_single(
        &mut self,
        anim: Animation,
        easing: Easing,
        delay_acc: &mut f32,
        ids: &mut Vec<usize>,
    ) {
        let (id, dur) = self.spawn_instance(anim, easing, *delay_acc);
        *delay_acc += dur;
        ids.push(id);
    }


    fn handle_parallel(
        &mut self,
        anims: Vec<(Animation, Easing)>,
        delay_acc: &mut f32,
        ids: &mut Vec<usize>,
    ) {
        let mut max_len: f32 = 0.0;

        for (anim, easing) in anims {
            let (id, dur) = self.spawn_instance(anim, easing, *delay_acc);
            max_len = max_len.max(dur);
            ids.push(id);
        }

        *delay_acc += max_len;
    }

}

impl Default for AnimationController {
    fn default() -> Self {
        Self::new()
    }
}