use std::time::Duration;

use gamedata::spells::SpellAnimation;

use crate::registry::ResourceRegistry;

use super::time_progress::TimeProgress;

pub struct EntityAnimation {
    pub position: (i32, i32),
    pub progress: TimeProgress,
    pub spell_animation: SpellAnimation,
}

pub struct EntityAnimations(pub Vec<EntityAnimation>);

impl EntityAnimations {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn update(&mut self, dt: Duration, _rr: &mut ResourceRegistry) {
        self.0.retain_mut(|a| {
            a.progress.update(dt);
            !a.progress.is_finished()
        })
    }

    pub fn push(&mut self, entity_animation: EntityAnimation) {
        self.0.push(entity_animation);
    }
}
