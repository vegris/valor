use std::collections::HashMap;
use std::path::Path;

use formats::snd::SndIndex;
use sdl2::mixer::{Chunk, LoaderRWops};
use sdl2::rwops::RWops;
use strum::EnumCount;

use formats::lod::LodIndex;
use gamedata::creatures::Creature;

use crate::error::AnyHow;

pub mod images;
pub mod spritesheets;

use self::images::{PaletteImage, StaticImage};
use self::spritesheets::{SpriteGroup, SpriteGroupType, SpriteSheet, SpriteSheetType};

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

    pub fn load_spritesheet<S: SpriteSheetType>(&mut self, filename: &str) -> SpriteSheet<S> {
        let bytes = self.def_archive.read_file(filename);
        SpriteSheet::from_bytes(bytes)
    }

    pub fn load_sound(&mut self, filename: &str) -> AnyHow<Chunk> {
        let bytes = self.snd_archive.read_file(filename);
        let chunk = RWops::from_bytes(&bytes)?.load_wav()?;
        Ok(chunk)
    }

    pub fn get_creature_spritesheet(
        &mut self,
        creature: Creature,
    ) -> &SpriteSheet<crate::graphics::creature::AnimationType> {
        if self.creature_cache.get(creature).is_none() {
            let spritesheet = self.load_spritesheet(creature.spritesheet_filename());
            self.creature_cache.put(creature, spritesheet);
        }
        self.creature_cache.get(creature).unwrap()
    }

    pub fn get_sound(&mut self, filename: &str) -> &Chunk {
        if self.sound_cache.get(filename).is_none() {
            let chunk = self.load_sound(filename).unwrap();
            self.sound_cache.put(filename, chunk);
        }
        self.sound_cache.get(filename).unwrap()
    }
}

type CachedValue = SpriteSheet<crate::graphics::creature::AnimationType>;

struct CreaturesCache([Option<CachedValue>; Creature::COUNT]);

impl CreaturesCache {
    fn new() -> Self {
        const NONE: Option<CachedValue> = None;
        Self([NONE; Creature::COUNT])
    }

    fn get(&mut self, creature: Creature) -> Option<&CachedValue> {
        self.0[creature as usize].as_ref()
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
