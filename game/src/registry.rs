use std::collections::HashMap;
use std::error::Error;

use std::path::Path;

use formats::snd::SndIndex;
use sdl2::mixer::{Chunk, LoaderRWops};
use sdl2::rwops::RWops;
use strum::EnumCount;

use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;

use formats::def::container::Container;
use formats::lod::LodIndex;
use formats::pcx;

use gamedata::creatures::Creature;

use crate::graphics::spritesheet::creature::AnimationType;
use crate::graphics::spritesheet::Spritesheet;

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

    pub fn load_pcx(&mut self, filename: &str) -> Result<Surface<'static>, Box<dyn Error>> {
        self.load_pcx_internal(filename, true)
    }

    pub fn load_pcx_with_transparency(
        &mut self,
        filename: &str,
    ) -> Result<Surface<'static>, Box<dyn Error>> {
        self.load_pcx_internal(filename, true)
    }

    fn load_pcx_internal(
        &mut self,
        filename: &str,
        apply_transparency: bool,
    ) -> Result<Surface<'static>, Box<dyn Error>> {
        let mut bytes = self.pcx_archive.read_file(filename);
        let image = pcx::from_bytes(&mut bytes)?;

        pcx_to_surface(image, apply_transparency)
    }

    pub fn load_def(&mut self, filename: &str) -> Container {
        let bytes = self.def_archive.read_file(filename);
        Container::from_bytes(&bytes)
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

fn pcx_to_surface(
    image: pcx::Image,
    apply_transparency: bool,
) -> Result<Surface<'static>, Box<dyn Error>> {
    let pcx::Image {
        width,
        height,
        data: mut image_data,
        ..
    } = image;

    let mut surface = {
        let (pixels, pitch, pixel_format) = match image_data {
            pcx::ImageData::RGB24(ref mut bytes) => (bytes, width * 3, PixelFormatEnum::BGR24),
            pcx::ImageData::Index8 { ref mut bytes, .. } => (bytes, width, PixelFormatEnum::Index8),
        };

        let surface = Surface::from_data(pixels, width, height, pitch, pixel_format)?;
        surface.convert_format(surface.pixel_format_enum())?
    };

    if let pcx::ImageData::Index8 { colors, .. } = image_data {
        let mut colors: Box<[Color]> = colors
            .iter()
            .map(|c| Color::RGB(c.red, c.green, c.blue))
            .collect();

        if apply_transparency {
            let color_changes = [0, 32, 64, 128, 128];

            for (index, alpha) in color_changes.into_iter().enumerate() {
                colors[index] = Color::RGBA(0, 0, 0, alpha);
            }

            surface.set_color_key(true, Color::RGB(0, 0, 0))?;
        }

        let palette = Palette::with_colors(&colors)?;
        surface.set_palette(&palette)?;
    }

    Ok(surface)
}

type CachedValue = Spritesheet<AnimationType>;

pub struct CreaturesCache([Option<CachedValue>; Creature::COUNT]);

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

pub struct SoundCache(HashMap<String, Chunk>);

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
