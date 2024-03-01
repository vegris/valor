use sdl2::mixer::Chunk;
use strum::EnumCount;

use gamedata::creatures;
use gamedata::creatures::Creature;

use crate::graphics::creature::AnimationType;

use super::spritesheets::SpriteSheet;

pub struct CreaturesCache([Option<CreatureResources>; Creature::COUNT]);

impl CreaturesCache {
    pub fn new() -> Self {
        const NONE: Option<CreatureResources> = None;

        Self([NONE; Creature::COUNT])
    }

    pub fn get(&self, creature: Creature) -> Option<&CreatureResources> {
        self.0[creature as usize].as_ref()
    }

    pub fn put(&mut self, creature: Creature, resources: CreatureResources) {
        self.0[creature as usize] = Some(resources);
    }
}

pub struct CreatureResources {
    spritesheet: SpriteSheet<AnimationType>,
    sounds: CreatureSounds,
}

impl CreatureResources {
    pub fn new(
        spritesheet: SpriteSheet<AnimationType>,
        sounds: [Option<Chunk>; creatures::Sound::COUNT],
    ) -> Self {
        Self {
            spritesheet,
            sounds: CreatureSounds(sounds),
        }
    }
    pub fn spritesheet(&self) -> &SpriteSheet<AnimationType> {
        &self.spritesheet
    }
    pub fn sounds(&self) -> &CreatureSounds {
        &self.sounds
    }
}

pub struct CreatureSounds([Option<Chunk>; creatures::Sound::COUNT]);

impl CreatureSounds {
    pub fn get(&self, sound_type: creatures::Sound) -> Option<&Chunk> {
        self.0[sound_type as usize].as_ref()
    }
}
