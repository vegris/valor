use strum::EnumCount;

use gamedata::spells::SpellAnimation;

use super::{spritesheets::SpriteSheet, SpellAnimationType};

type SpellSpritesheet = SpriteSheet<SpellAnimationType>;
pub struct SpellsCache(pub(crate) [Option<SpellSpritesheet>; SpellAnimationType::COUNT]);

impl SpellsCache {
    pub fn new() -> Self {
        const NONE: Option<SpellSpritesheet> = None;

        Self([NONE; SpellAnimation::COUNT])
    }

    pub fn get(&self, animation: SpellAnimation) -> Option<&SpellSpritesheet> {
        self.0[animation as usize].as_ref()
    }

    pub fn put(&mut self, animation: SpellAnimation, spritesheet: SpellSpritesheet) {
        self.0[animation as usize] = Some(spritesheet);
    }
}
