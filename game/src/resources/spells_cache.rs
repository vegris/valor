use strum::EnumCount;

use gamedata::spells::SpellAnimation;

use super::spritesheets::SpriteSheetSingle;

pub struct SpellsCache(pub(crate) [Option<SpriteSheetSingle>; SpellAnimation::COUNT]);

impl SpellsCache {
    pub fn new() -> Self {
        const NONE: Option<SpriteSheetSingle> = None;

        Self([NONE; SpellAnimation::COUNT])
    }

    pub fn get(&self, animation: SpellAnimation) -> Option<&SpriteSheetSingle> {
        self.0[animation as usize].as_ref()
    }

    pub fn put(&mut self, animation: SpellAnimation, spritesheet: SpriteSheetSingle) {
        self.0[animation as usize] = Some(spritesheet);
    }
}
