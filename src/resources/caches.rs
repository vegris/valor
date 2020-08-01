use std::mem::MaybeUninit;

extern crate sdl2;

use crate::enumerations::Creature;
use super::creature_spritesheet::CreatureSpritesheet;

type CachedValue = CreatureSpritesheet;

pub struct CreaturesCache {
    cache: [Option<CachedValue>; Creature::count()]
}

impl CreaturesCache {
    pub fn new() -> Self {
        let mut cache: [MaybeUninit<Option<CachedValue>>; Creature::count()] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for elem in &mut cache[..] {
            *elem = MaybeUninit::new(None);
        }
        let cache = unsafe { std::mem::transmute::<_, [Option<CachedValue>; Creature::count()]>(cache) };
        Self { cache }
    }
    
    pub fn get(&mut self, creature: Creature) -> Option<&mut CachedValue> {
        self.cache[creature as usize].as_mut()
    }

    pub fn put(&mut self, creature: Creature, value: CachedValue) {
        self.cache[creature as usize] = Some(value);
    }
}