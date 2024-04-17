use sdl2::mixer::Chunk;
use strum::EnumCount;

use gamedata::creatures;

use super::spritesheets::AnimationGroup;

pub struct CreatureResources {
    spritesheet: AnimationGroup<creatures::Animation>,
    sounds: CreatureSounds,
}

impl CreatureResources {
    pub fn new(
        spritesheet: AnimationGroup<creatures::Animation>,
        sounds: [Option<Chunk>; creatures::Sound::COUNT],
    ) -> Self {
        Self {
            spritesheet,
            sounds: CreatureSounds(sounds),
        }
    }
    pub fn spritesheet(&self) -> &AnimationGroup<creatures::Animation> {
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
