use std::time::Duration;

use gamedata::creatures::Creature;

use crate::{graphics::spritesheet::creature::AnimationType, registry::ResourceRegistry};

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
        let spritesheet = rr.get_creature_container(creature);

        let frame_count = spritesheet.frames_count(animation_type).unwrap();
        let duration = frame_duration(animation_type) * frame_count as u32;

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

fn frame_duration(animation_type: AnimationType) -> Duration {
    let ms = match animation_type {
        AnimationType::Standing => 200,
        AnimationType::TurnLeft | AnimationType::TurnRight => 100,
        AnimationType::Moving => 100,
        _ => 100,
    };

    Duration::from_millis(ms)
}
