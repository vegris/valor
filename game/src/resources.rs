use std::marker::PhantomData;

use common::EnumIndex;
use gamedata::traits::{AnimationGroupT, ContainerType, SpriteGroupT};
use sdl2::mixer::{Chunk, LoaderRWops};
use sdl2::rwops::RWops;

use strum::{EnumCount, IntoEnumIterator};

use formats::def;
use formats::lod::LodIndex;
use formats::snd::SndIndex;

use gamedata::creatures;
use gamedata::creatures::Creature;
use gamedata::spells::SpellAnimation;

use common::error::AnyHow;

mod creature_resources;
pub mod images;
pub mod spritesheets;

use self::creature_resources::CreatureResources;
use self::images::{PaletteImage, StaticImage};
use self::spritesheets::{AnimationGroup, SpriteGroup, SpriteSheetSingle};

const PCX_ARCHIVE: &str = "H3bitmap.lod";
const DEF_ARCHIVE: &str = "H3sprite.lod";
const SND_ARCHIVE: &str = "Heroes3.snd";

pub struct ResourceRegistry {
    pcx_archive: LodIndex,
    def_archive: LodIndex,
    snd_archive: SndIndex,
    creature_cache: ResourceCache<{ Creature::COUNT }, Creature, CreatureResources>,
    spells_cache: ResourceCache<{ SpellAnimation::COUNT }, SpellAnimation, SpriteSheetSingle>,
}

struct ResourceCache<const SIZE: usize, I: EnumIndex, V> {
    cache: [Option<V>; SIZE],
    index: PhantomData<I>,
}

impl ResourceRegistry {
    pub fn init() -> Self {
        let pcx_archive = LodIndex::open(PCX_ARCHIVE);
        let def_archive = LodIndex::open(DEF_ARCHIVE);
        let snd_archive = SndIndex::open(SND_ARCHIVE);

        ResourceRegistry {
            pcx_archive,
            def_archive,
            snd_archive,
            creature_cache: ResourceCache::new(),
            spells_cache: ResourceCache::new(),
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

    pub fn load_sprite_group<G: SpriteGroupT>(&mut self, filename: &str) -> SpriteGroup<G> {
        SpriteGroup::from_def(self.load_def(filename))
    }

    pub fn load_spritesheet<S: AnimationGroupT>(&mut self, filename: &str) -> AnimationGroup<S> {
        AnimationGroup::from_def(self.load_def(filename))
    }

    pub fn load_animation<const T: u32>(&mut self, filename: &str) -> SpriteSheetSingle {
        SpriteSheetSingle::from_def::<T>(self.load_def(filename))
    }

    fn load_def(&mut self, filename: &str) -> def::Container {
        let bytes = self.def_archive.read_file(filename);
        def::Container::from_bytes(&bytes)
    }

    pub fn load_sound(&mut self, filename: &str) -> AnyHow<Chunk> {
        let bytes = self.snd_archive.read_file(filename);
        let chunk = RWops::from_bytes(&bytes)?.load_wav()?;
        Ok(chunk)
    }

    pub fn get_creature_spritesheet(
        &mut self,
        creature: Creature,
    ) -> &AnimationGroup<creatures::Animation> {
        self.get_creature_resources(creature).spritesheet()
    }

    pub fn get_creature_sound(
        &mut self,
        creature: Creature,
        sound: creatures::Sound,
    ) -> Option<&Chunk> {
        self.get_creature_resources(creature).sounds().get(sound)
    }

    fn get_creature_resources(&mut self, creature: Creature) -> &CreatureResources {
        if self.creature_cache.get(creature).is_none() {
            let resources = self.load_creature_resources(creature);
            self.creature_cache.put(creature, resources);
        }
        self.creature_cache.get(creature).unwrap()
    }

    fn load_creature_resources(&mut self, creature: Creature) -> CreatureResources {
        let spritesheet = self.load_spritesheet(creature.spritesheet_filename());

        let sounds = creatures::Sound::iter()
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

    pub fn get_spell_animation(&mut self, spell_animation: SpellAnimation) -> &SpriteSheetSingle {
        if self.spells_cache.get(spell_animation).is_none() {
            let spritesheet = self.load_animation::<{ SpellAnimation::CONTAINER_TYPE }>(
                spell_animation.spritesheet(),
            );
            self.spells_cache.put(spell_animation, spritesheet);
        }
        self.spells_cache.get(spell_animation).unwrap()
    }
}

impl<const SIZE: usize, I: EnumIndex, V> ResourceCache<{ SIZE }, I, V> {
    const NONE: Option<V> = None;

    fn new() -> Self {
        Self {
            cache: [Self::NONE; SIZE],
            index: PhantomData,
        }
    }

    fn get(&self, index: I) -> Option<&V> {
        self.cache[index.into_index()].as_ref()
    }

    fn put(&mut self, index: I, value: V) {
        self.cache[index.into_index()] = Some(value);
    }
}
