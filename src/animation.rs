use std::time::{Instant, Duration};

use crate::enumerations::Creature;
use crate::resources::{ResourceRegistry, AnimationType, CreatureSprite};

const ADVANCE_PERIODITY: Duration = Duration::from_millis(128);

pub struct Animation {
    advance_at: Option<Instant>,
    cur_index: usize,
    creature: Creature,
    animation_type: AnimationType,
    animation_length: usize
}

impl Animation {
    pub fn new(creature: Creature, animation_type: AnimationType, rr: &mut ResourceRegistry) -> Self {
        let animation_length = rr.get_creature_container(creature).get_animation_block(animation_type).unwrap().len();
        Self {
            advance_at: None,
            cur_index: 0,
            creature,
            animation_type,
            animation_length
        }
    }

    pub fn default(creature: Creature, rr: &mut ResourceRegistry) -> Self {
        Self::new(creature, AnimationType::Standing, rr)
    }

    pub fn update(&mut self, now: Instant) {
        if let Some(advance_at) = self.advance_at {
            if now >= advance_at {
                if self.cur_index == self.animation_length - 1  {
                    self.cur_index = 0;
                } else {
                    self.cur_index += 1;
                }
                let advance_at = now + (now - advance_at) + ADVANCE_PERIODITY;
                self.advance_at = Some(advance_at);
            }
        } else {
            // Не начатая анимация
            self.advance_at = Some(now + ADVANCE_PERIODITY);
        }
    }

    pub fn get_sprite<'a>(&self, rr: &'a mut ResourceRegistry) -> &'a CreatureSprite {
        rr.get_creature_container(self.creature).get_sprite(self.animation_type, self.cur_index).unwrap()
    }
}