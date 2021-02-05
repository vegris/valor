use std::mem::MaybeUninit;

use creature::Creature;
use crate::graphics::creature::CreatureSpritesheet;

type CachedValue = CreatureSpritesheet;

pub struct CreaturesCache {
    cache: [Option<CachedValue>; Creature::COUNT]
}

impl CreaturesCache {
    pub fn new() -> Self {
        let mut cache: [MaybeUninit<Option<CachedValue>>; Creature::COUNT] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for elem in &mut cache[..] {
            *elem = MaybeUninit::new(None);
        }
        let cache = unsafe { std::mem::transmute::<_, [Option<CachedValue>; Creature::COUNT]>(cache) };
        Self { cache }
    }
    
    pub fn get(&mut self, creature: Creature) -> Option<&mut CachedValue> {
        self.cache[creature as usize].as_mut()
    }

    pub fn put(&mut self, creature: Creature, value: CachedValue) {
        self.cache[creature as usize] = Some(value);
    }
}