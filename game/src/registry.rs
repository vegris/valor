use std::error::Error;
use std::path::Path;

extern crate sdl2;
use sdl2::surface::Surface;

use formats::{pcx, DefContainer, LodIndex};
use gamedata::Creature;

use crate::graphics::creature;

const PCX_ARCHIVE: &str = "H3bitmap.lod";
const DEF_ARCHIVE: &str = "H3sprite.lod";

pub struct ResourceRegistry {
    pcx_archive: LodIndex,
    def_archive: LodIndex,
    cache: CreaturesCache,
}

impl ResourceRegistry {
    pub fn init() -> Self {
        let pcx_archive = LodIndex::open(Path::new(PCX_ARCHIVE));
        let def_archive = LodIndex::open(Path::new(DEF_ARCHIVE));

        let cache = CreaturesCache::new();

        ResourceRegistry {
            pcx_archive,
            def_archive,
            cache,
        }
    }

    pub fn load_pcx(&mut self, filename: &str) -> Result<Surface<'static>, Box<dyn Error>> {
        let mut bytes = self.pcx_archive.read_file(filename);
        let pcx = pcx::from_bytes(&mut bytes)?;
        either::for_both!(pcx, img => img.to_surface())
    }
    pub fn load_pcx_with_transparency(
        &mut self,
        filename: &str,
    ) -> Result<Surface<'static>, Box<dyn Error>> {
        let mut bytes = self.pcx_archive.read_file(filename);
        let pcx = pcx::from_bytes(&mut bytes)?;

        let mut image8 = pcx
            .right()
            .ok_or::<String>("Unexpected image type!".into())?;

        image8.apply_transparency()?;
        image8.to_surface()
    }

    pub fn load_def(&mut self, filename: &str) -> DefContainer {
        let bytes = self.def_archive.read_file(filename);
        DefContainer::from_bytes(&bytes)
    }

    pub fn get_creature_container(&mut self, creature: Creature) -> &mut creature::Spritesheet {
        if self.cache.get(creature).is_none() {
            let def = self.load_def(creature.spritesheet_filename());
            let spritesheet = creature::Spritesheet::from_def_container(def);
            self.cache.put(creature, spritesheet);
        }
        self.cache.get(creature).unwrap()
    }
}

type CachedValue = creature::Spritesheet;

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
