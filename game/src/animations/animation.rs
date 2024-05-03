use gamedata::creatures;
use gamedata::creatures::Creature;

use super::time_progress::TimeProgress;
use crate::resources::ResourceRegistry;

pub struct Animation {
    pub type_: creatures::Animation,
    pub frame_count: usize,
    progress: TimeProgress,
}

impl Animation {
    pub fn new(
        animation_type: creatures::Animation,
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

    pub fn progress(&self) -> &TimeProgress {
        &self.progress
    }

    pub fn progress_mut(&mut self) -> &mut TimeProgress {
        &mut self.progress
    }
}
