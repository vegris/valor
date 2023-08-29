use std::collections::HashMap;
use std::path::Path;

use formats::snd::SndIndex;
use sdl2::mixer::{Chunk, LoaderRWops};
use sdl2::rwops::RWops;
use strum::EnumCount;

use formats::def::Container;
use formats::lod::LodIndex;

use gamedata::creatures::Creature;

use crate::error::AnyHow;
use crate::graphics::spritesheet::creature::AnimationType;
use crate::graphics::spritesheet::Spritesheet;

pub mod def;
pub mod images;
pub mod spritesheets;

use self::images::{PaletteImage, StaticImage};
use self::spritesheets::{SpriteGroup, SpriteGroupType};

const PCX_ARCHIVE: &str = "H3bitmap.lod";
const DEF_ARCHIVE: &str = "H3sprite.lod";
const SND_ARCHIVE: &str = "Heroes3.snd";

pub struct ResourceRegistry {
    pcx_archive: LodIndex,
    def_archive: LodIndex,
    snd_archive: SndIndex,
    creature_cache: CreaturesCache,
    sound_cache: SoundCache,
}

impl ResourceRegistry {
    pub fn init() -> Self {
        let pcx_archive = LodIndex::open(Path::new(PCX_ARCHIVE));
        let def_archive = LodIndex::open(Path::new(DEF_ARCHIVE));
        let snd_archive = SndIndex::open(Path::new(SND_ARCHIVE));

        ResourceRegistry {
            pcx_archive,
            def_archive,
            snd_archive,
            creature_cache: CreaturesCache::new(),
            sound_cache: SoundCache::new(),
        }
    }

    pub fn load_static_image(&mut self, filename: &str) -> AnyHow<StaticImage> {
        let bytes = self.pcx_archive.read_file(filename);
        let image = StaticImage::from_bytes(bytes)?;
        Ok(image)
    }

    pub fn load_palette_image(&mut self, filename: &str) -> AnyHow<PaletteImage> {
        let bytes = self.pcx_archive.read_file(filename);
        let image = PaletteImage::from_bytes(bytes)?;
        Ok(image)
    }

    pub fn load_sprite_group<G: SpriteGroupType>(&mut self, filename: &str) -> SpriteGroup<G> {
        let bytes = self.def_archive.read_file(filename);
        SpriteGroup::from_bytes(bytes)
    }

    pub fn load_def(&mut self, filename: &str) -> def::Container {
        let bytes = self.def_archive.read_file(filename);
        let raw = Container::from_bytes(&bytes);
        def::Container::from_raw(raw)
    }

    pub fn get_creature_container(
        &mut self,
        creature: Creature,
    ) -> &mut Spritesheet<AnimationType> {
        if self.creature_cache.get(creature).is_none() {
            let def = self.load_def(creature.spritesheet_filename());
            let spritesheet = Spritesheet::from_def(def);
            self.creature_cache.put(creature, spritesheet);
        }
        self.creature_cache.get(creature).unwrap()
    }

    pub fn get_sound(&mut self, filename: &str) -> &Chunk {
        if self.sound_cache.get(filename).is_none() {
            let bytes = self.snd_archive.read_file(filename);
            let chunk = RWops::from_bytes(&bytes).unwrap().load_wav().unwrap();
            self.sound_cache.put(filename, chunk);
        }
        self.sound_cache.get(filename).unwrap()
    }
}

type CachedValue = Spritesheet<AnimationType>;

struct CreaturesCache([Option<CachedValue>; Creature::COUNT]);

impl CreaturesCache {
    fn new() -> Self {
        const NONE: Option<CachedValue> = None;
        Self([NONE; Creature::COUNT])
    }

    fn get(&mut self, creature: Creature) -> Option<&mut CachedValue> {
        self.0[creature as usize].as_mut()
    }

    fn put(&mut self, creature: Creature, value: CachedValue) {
        self.0[creature as usize] = Some(value);
    }
}

struct SoundCache(HashMap<String, Chunk>);

impl SoundCache {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get(&mut self, filename: &str) -> Option<&Chunk> {
        self.0.get(filename)
    }

    fn put(&mut self, filename: &str, sound: Chunk) {
        self.0.insert(filename.to_owned(), sound);
    }
}
