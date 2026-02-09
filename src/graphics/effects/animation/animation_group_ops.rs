use crate::graphics::effects::animation::animation_system::AnimationSystem;
use crate::graphics::effects::animation::AnimationGroupID;
use crate::prelude::{Animation, Easing};

/// Extension methods for `AnimationSystem` that operate on groups of
/// animations (sequence, parallel, timeline).
impl AnimationSystem {
    pub fn start_sequence(
        &mut self,
        steps: Vec<(Animation, Easing)>,
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
        steps: Vec<(Animation, Easing)>,
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

    pub fn stop_by_group(&mut self, group: &AnimationGroupID) {
        for id in group.iter() {
            self.stop(*id);
        }
    }

    pub fn remove_from_group(&mut self, group: &mut AnimationGroupID, index: usize) {
        if let Some(&id) = group.get_id(index) { self.stop(id); }
        group.remove(index);
    }

    #[must_use]
    pub fn is_group_playing_all(&self, group: &AnimationGroupID) -> bool {
        group.iter().all(|id| self.is_playing_id(*id))
    }

    #[must_use]
    pub fn is_group_finished_all(&self, group: &AnimationGroupID) -> bool {
        group.iter().all(|id| !self.is_playing_id(*id))
    }

    #[must_use]
    pub fn is_group_playing_any(&self, group: &AnimationGroupID) -> bool {
        group.iter().any(|id| self.is_playing_id(*id))
    }

    #[must_use]
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

    pub fn restart_group(&mut self, group: &AnimationGroupID) {
        for id in group.iter() {
            self.seek_progress(*id, 0.0);
        }
    }
}
