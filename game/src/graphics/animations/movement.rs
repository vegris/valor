use std::time::Duration;

use sdl2::rect::Point;

use gamedata::creatures::Creature;

use crate::graphics::creature::AnimationType;
use crate::grid::GridPos;
use crate::ResourceRegistry;

use super::time_progress::TimeProgress;

pub struct Movement {
    tweens: Box<[Tween]>,
    frame_count: usize,
    modifier: f32,
    progress: TimeProgress,
}

struct Tween {
    from: Point,
    to: Point,
}

impl Movement {
    pub const ANIMATION_TYPE: AnimationType = AnimationType::Moving;

    pub fn new(creature: Creature, path: Vec<GridPos>, rr: &mut ResourceRegistry) -> Self {
        let tweens = if creature.is_flying() {
            vec![Tween {
                from: path[0].center(),
                to: path.last().unwrap().center(),
            }]
            .into_boxed_slice()
        } else {
            let iter1 = path.iter().map(|p| p.center());
            let iter2 = iter1.clone().skip(1);
            Iterator::zip(iter1, iter2)
                .map(|(from, to)| Tween { from, to })
                .collect()
        };

        let spritesheet = rr.get_creature_spritesheet(creature);
        let frame_count = spritesheet.frames_count(Self::ANIMATION_TYPE).unwrap();

        let duration = total_duration(creature, &tweens);

        Self {
            tweens,
            modifier: creature.walk_animation_modifier(),
            frame_count,
            progress: TimeProgress::new(duration),
        }
    }

    pub fn get_position(&self) -> Point {
        let tween_index_f = self.tweens.len() as f32 * self.progress.progress().min(0.99);

        let tween_index = tween_index_f.floor() as usize;
        let tween_progress = tween_index_f.fract();

        let Tween { from, to } = &self.tweens[tween_index];

        from.offset(
            ((to.x - from.x) as f32 * tween_progress) as i32,
            ((to.y - from.y) as f32 * tween_progress) as i32,
        )
    }

    pub fn get_frame(&self) -> usize {
        let animation_duration = (Self::ANIMATION_TYPE.frame_duration() * self.frame_count as u32)
            .as_secs_f32()
            * self.modifier;

        let progress = self.progress.spent.as_secs_f32() % animation_duration / animation_duration;

        ((self.frame_count - 1) as f32 * progress).round() as usize
    }

    pub fn progress(&self) -> &TimeProgress {
        &self.progress
    }

    pub fn progress_mut(&mut self) -> &mut TimeProgress {
        &mut self.progress
    }
}

impl Tween {
    fn length_px(&self) -> i32 {
        let a = (self.to.x - self.from.x).abs();
        let b = (self.to.y - self.from.y).abs();

        ((a.pow(2) + b.pow(2)) as f32).sqrt().round() as i32
    }
}

const FLIGHT_PX_PER_SECOND: i32 = 250;
const WALK_TILE_PER_SECOND: i32 = 2;

fn total_duration(creature: Creature, tweens: &[Tween]) -> Duration {
    let modifier = creature.walk_animation_modifier();

    let duration_secs = if creature.is_flying() {
        let length: i32 = tweens.iter().map(|t| t.length_px()).sum();

        length as f32 / FLIGHT_PX_PER_SECOND as f32 / modifier
    } else {
        tweens.len() as f32 / WALK_TILE_PER_SECOND as f32 / modifier
    };

    Duration::from_secs_f32(duration_secs)
}
