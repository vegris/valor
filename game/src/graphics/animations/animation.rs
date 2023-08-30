use gamedata::creatures::Creature;

use crate::graphics::creature::AnimationType;
use crate::registry::ResourceRegistry;

use super::TimeProgress;

pub struct Animation {
    pub type_: AnimationType,
    pub frame_count: usize,
    pub progress: TimeProgress,
}

impl Animation {
    pub fn new(
        animation_type: AnimationType,
        creature: Creature,
        rr: &mut ResourceRegistry,
    ) -> Self {
        let spritesheet = rr.get_creature_spritesheet(creature);

        let frame_count = spritesheet.frames_count(animation_type).unwrap();
        let duration = animation_type.frame_duration() * frame_count as u32;

        Self {
            type_: animation_type,
            frame_count,
            progress: TimeProgress::new(duration),
        }
    }

    pub fn get_frame(&self) -> usize {
        let progress = self.progress.progress();

        ((self.frame_count - 1) as f32 * progress).round() as usize
    }
}
