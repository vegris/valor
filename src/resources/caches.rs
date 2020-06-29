use std::mem::MaybeUninit;

extern crate sdl2;
use sdl2::surface::Surface;

use crate::enumerations::Battlefield;

pub struct BattlefieldsCache {
    cache: [Option<Surface<'static>>; Battlefield::count()]
}

impl BattlefieldsCache {
    pub fn new() -> Self {
        let mut cache: [MaybeUninit<Option<Surface<'static>>>; Battlefield::count()] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        for elem in &mut cache[..] {
            *elem = MaybeUninit::new(None);
        }
        let cache = unsafe { std::mem::transmute::<_, [Option<Surface<'static>>; Battlefield::count()]>(cache) };
        Self { cache }
    }
    
    pub fn get(&self, battlefield: Battlefield) -> Option<&Surface<'static>> {
        self.cache[battlefield as usize].as_ref()
    }

    pub fn put(&mut self, battlefield: Battlefield, surface: Surface<'static>) {
        self.cache[battlefield as usize] = Some(surface);
    }
}