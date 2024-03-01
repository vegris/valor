use std::path::Path;

use gamedata::spells::SpellAnimation;
use sdl2::mixer::{Chunk, LoaderRWops};
use sdl2::rwops::RWops;
use strum::IntoEnumIterator;

use formats::lod::LodIndex;
use formats::snd::SndIndex;
use gamedata::creatures::{Creature, CreatureSound};
use strum_macros::{EnumCount, EnumIter};

use crate::error::AnyHow;

mod creature_resources;
pub mod images;
mod spells_cache;
pub mod spritesheets;

use self::creature_resources::{CreatureResources, CreaturesCache};
use self::images::{PaletteImage, StaticImage};
use self::spells_cache::SpellsCache;
use self::spritesheets::{
    ContainerType, SpriteGroup, SpriteGroupType, SpriteSheet, SpriteSheetType,
};

const PCX_ARCHIVE: &str = "H3bitmap.lod";
const DEF_ARCHIVE: &str = "H3sprite.lod";
const SND_ARCHIVE: &str = "Heroes3.snd";

pub struct ResourceRegistry {
    pcx_archive: LodIndex,
    def_archive: LodIndex,
    snd_archive: SndIndex,
    creatures_cache: CreaturesCache,
    spells_cache: SpellsCache,
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
            creatures_cache: CreaturesCache::new(),
            spells_cache: SpellsCache::new(),
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
        self.get_creature_resources(creature).spritesheet()
    }

    pub fn get_creature_sound(
        &mut self,
        creature: Creature,
        sound: CreatureSound,
    ) -> Option<&Chunk> {
        self.get_creature_resources(creature).sounds().get(sound)
    }

    fn get_creature_resources(&mut self, creature: Creature) -> &CreatureResources {
        if self.creatures_cache.get(creature).is_none() {
            let resources = self.load_creature_resources(creature);
            self.creatures_cache.put(creature, resources);
        }
        self.creatures_cache.get(creature).unwrap()
    }

    fn load_creature_resources(&mut self, creature: Creature) -> CreatureResources {
        let spritesheet = self.load_spritesheet(creature.spritesheet_filename());

        let sounds = CreatureSound::iter()
            .map(|sound| {
                if let Some(filename) = creature.sounds().get(sound) {
                    let chunk = self.load_sound(filename).unwrap();
                    Some(chunk)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();

        CreatureResources::new(spritesheet, sounds)
    }

    pub fn get_spell_animation(
        &mut self,
        spell_animation: SpellAnimation,
    ) -> &SpriteSheet<SpellAnimationType> {
        if self.spells_cache.get(spell_animation).is_none() {
            let spritesheet = self.load_spritesheet(spell_animation.spritesheet());
            self.spells_cache.put(spell_animation, spritesheet);
        }
        self.spells_cache.get(spell_animation).unwrap()
    }
}

#[derive(Clone, Copy, EnumCount, EnumIter)]
pub enum SpellAnimationType {
    Casting,
}

impl ContainerType for SpellAnimationType {
    const CONTAINER_TYPE: u32 = 64;
}
impl SpriteSheetType for SpellAnimationType {
    fn block_index(&self) -> usize {
        *self as usize
    }

    fn container_index(&self) -> u32 {
        0
    }
}
