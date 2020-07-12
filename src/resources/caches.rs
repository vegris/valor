use std::mem::MaybeUninit;

extern crate sdl2;

use crate::enumerations::Creature;
use super::formats::DefContainer;

pub struct CreaturesCache {
    cache: [Option<DefContainer>; Creature::count()]
}

impl CreaturesCache {
    pub fn new() -> Self {
        let mut cache: [MaybeUninit<Option<DefContainer>>; Creature::count()] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for elem in &mut cache[..] {
            *elem = MaybeUninit::new(None);
        }
        let cache = unsafe { std::mem::transmute::<_, [Option<DefContainer>; Creature::count()]>(cache) };
        Self { cache }
    }
    
    pub fn get(&mut self, creature: Creature) -> Option<&mut DefContainer> {
        self.cache[creature as usize].as_mut()
    }

    pub fn put(&mut self, creature: Creature, def: DefContainer) {
        self.cache[creature as usize] = Some(def);
    }
}