use std::time::Duration;

use gamedata::creatures::Creature;
use sdl2::rect::Point;

use crate::{graphics::spritesheet::creature::AnimationType, registry::ResourceRegistry};

use super::TimeProgress;

pub struct Animation {
    pub type_: AnimationType,
    pub frame_count: usize,
    pub progress: TimeProgress,
    pub tween: Option<Tween>,
}

pub struct Tween {
    from: Point,
    to: Point,
}

pub struct AnimationData {
    pub frame_index: usize,
    pub position: Option<Point>,
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
            tween: None,
        }
    }

    pub fn get_state(&self) -> AnimationData {
        let progress = self.progress.progress();

        let frame_index = ((self.frame_count - 1) as f32 * progress).round() as usize;

        let position = self.tween.as_ref().map(|tween| {
            let x = tween.from.x + ((tween.to.x - tween.from.x) as f32 * progress) as i32;
            let y = tween.from.y + ((tween.to.y - tween.from.y) as f32 * progress) as i32;
            Point::new(x, y)
        });

        AnimationData {
            frame_index,
            position,
        }
    }
}

fn frame_duration(animation_type: AnimationType) -> Duration {
    let ms = match animation_type {
        AnimationType::Standing => 200,
        AnimationType::TurnLeft | AnimationType::TurnRight => 100,
        AnimationType::Moving => 50,
        _ => 100,
    };

    Duration::from_millis(ms)
}
