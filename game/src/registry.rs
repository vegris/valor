use std::path::Path;
use std::error::Error;

extern crate sdl2;
use sdl2::surface::Surface;

use creature::Creature;

use crate::graphics::creature::CreatureSpritesheet;

use formats::{LodIndex, PcxImage, DefContainer};

const PCX_ARCHIVE: &str = "H3bitmap.lod";
const DEF_ARCHIVE: &str = "H3sprite.lod";


pub struct ResourceRegistry {
    pcx_archive: LodIndex,
    def_archive: LodIndex,
    cache: CreaturesCache
}

impl ResourceRegistry {
    pub fn init() -> Self {
        let pcx_archive = LodIndex::open(Path::new(PCX_ARCHIVE));
        let def_archive = LodIndex::open(Path::new(DEF_ARCHIVE));

        let cache = CreaturesCache::new();
        
        ResourceRegistry {
            pcx_archive,
            def_archive,
            cache
        }
    }
    
    pub fn load_pcx(&mut self, filename: &str) -> Result<Surface<'static>, Box<dyn Error>> {
        let bytes = self.pcx_archive.read_file(filename);
        let pcx = PcxImage::from_bytes(bytes)?;
        pcx.to_surface()
    }
    pub fn load_pcx_with_transparency(&mut self, filename: &str) -> Result<Surface<'static>, Box<dyn Error>> {
        let bytes = self.pcx_archive.read_file(filename);
        let mut pcx = PcxImage::from_bytes(bytes)?;
        pcx.apply_transparency();
        pcx.to_surface()
    }

    pub fn load_def(&mut self, filename: &str) -> DefContainer {
        let bytes = self.def_archive.read_file(filename);
        DefContainer::from_bytes(bytes)
    }

    pub fn get_creature_container(&mut self, creature: Creature) -> &mut CreatureSpritesheet {
        if self.cache.get(creature).is_none() {
            let def = self.load_def(creature.spritesheet_filename());
            let spritesheet = CreatureSpritesheet::from_def_container(def);
            self.cache.put(creature, spritesheet);
        }
        self.cache.get(creature).unwrap()
    }
}


type CachedValue = CreatureSpritesheet;

pub struct CreaturesCache([Option<CachedValue>; Creature::COUNT]);

impl CreaturesCache {
    pub fn new() -> Self {
        const NONE: Option<CachedValue> = None;
        Self([NONE; Creature::COUNT])
    }
    
    pub fn get(&mut self, creature: Creature) -> Option<&mut CachedValue> {
        self.0[creature as usize].as_mut()
    }

    pub fn put(&mut self, creature: Creature, value: CachedValue) {
        self.0[creature as usize] = Some(value);
    }
}