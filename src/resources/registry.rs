use std::path::PathBuf;

extern crate sdl2;
use sdl2::surface::Surface;

use crate::util::AnyError;
use crate::enumerations::Creature;

use super::formats::{LodIndex, PcxImage, DefContainer};
use super::caches::CreaturesCache;
use super::creature_spritesheet::CreatureSpritesheet;


const RESOURCES_ROOT: &str = "/home/vsevolod/Wine/HoMM3/drive_c/HoMM3/Data";
const PCX_ARCHIVE: &str = "H3bitmap.lod";
const DEF_ARCHIVE: &str = "H3sprite.lod";


struct Caches {
    creatures: CreaturesCache
}

pub struct ResourceRegistry {
    pcx_archive: LodIndex,
    def_archive: LodIndex,
    caches: Caches
}

impl ResourceRegistry {
    pub fn init() -> Self {
        let pcx_archive = LodIndex::open(&[RESOURCES_ROOT, PCX_ARCHIVE].iter().collect::<PathBuf>());
        let def_archive = LodIndex::open(&[RESOURCES_ROOT, DEF_ARCHIVE].iter().collect::<PathBuf>());
        let caches = Caches {
            creatures: CreaturesCache::new()
        }; 
        
        ResourceRegistry {
            pcx_archive,
            def_archive,
            caches
        }
    }
    
    pub fn load_pcx(&mut self, filename: &str) -> Result<Surface<'static>, AnyError> {
        let bytes = self.pcx_archive.read_file(filename);
        let pcx = PcxImage::from_bytes(bytes)?;
        pcx.to_surface()
    }
    pub fn load_pcx_with_transparency(&mut self, filename: &str) -> Result<Surface<'static>, AnyError> {
        let bytes = self.pcx_archive.read_file(filename);
        let mut pcx = PcxImage::from_bytes(bytes)?;
        pcx.apply_transparency()?;
        pcx.to_surface()
    }

    fn load_def(&mut self, filename: &str) -> DefContainer {
        let bytes = self.def_archive.read_file(filename);
        DefContainer::from_bytes(bytes)
    }

    pub fn get_creature_container(&mut self, creature: Creature) -> &mut CreatureSpritesheet {
        if self.caches.creatures.get(creature).is_none() {
            let def = self.load_def(creature.filename());
            let spritesheet = CreatureSpritesheet::from_def_container(def);
            self.caches.creatures.put(creature, spritesheet);
        }
        self.caches.creatures.get(creature).unwrap()
    }
}
