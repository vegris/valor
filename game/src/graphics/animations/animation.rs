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

    pub fn new_with_tween(
        animation_type: AnimationType,
        creature: Creature,
        rr: &mut ResourceRegistry,
        tween: Tween,
    ) -> Self {
        let spritesheet = rr.get_creature_container(creature);

        let frame_count = spritesheet.frames_count(animation_type).unwrap();
        let duration = frame_duration(animation_type) * frame_count as u32;

        Self {
            type_: animation_type,
            frame_count,
            progress: TimeProgress::new(duration),
            tween: Some(tween),
        }
    }

    pub fn get_frame(&self) -> usize {
        let progress = self.progress.progress();

        ((self.frame_count - 1) as f32 * progress).round() as usize
    }

    pub fn get_position(&self) -> Option<Point> {
        let progress = self.progress.progress();

        self.tween.as_ref().map(|tween| {
            let x = tween.from.x + ((tween.to.x - tween.from.x) as f32 * progress) as i32;
            let y = tween.from.y + ((tween.to.y - tween.from.y) as f32 * progress) as i32;
            Point::new(x, y)
        })
    }
}

impl Tween {
    pub fn new(from: Point, to: Point) -> Self {
        Self { from, to }
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
