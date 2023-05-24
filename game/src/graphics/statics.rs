use std::error::Error;

use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::{Config, ResourceRegistry};

use super::cursors::Cursors;

pub struct Statics<'a> {
    pub(super) cursors: Cursors,
    pub(super) font: Font<'a, 'static>,
    pub(super) textures: Textures<'a>,
}

impl<'a> Statics<'a> {
    pub fn init(
        config: &Config,
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) -> Result<Self, Box<dyn Error>> {
        let font_path = "/usr/share/fonts/TTF/OpenSans-Bold.ttf";
        let font_size = 16;

        Ok(Self {
            cursors: Cursors::load(rr),
            font: ttf_context.load_font(font_path, font_size)?,
            textures: Textures::load(config, rr, tc)?,
        })
    }
}

#[derive(Clone, Copy)]
pub enum StaticTexture {
    Battlefield = 0,
    StackCountBackground = 1,
    GridCell = 2,
    GridCellShadow = 3,
}

pub struct Textures<'a>([Texture<'a>; 4]);

impl<'a> Textures<'a> {
    fn load(
        config: &Config,
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let textures: Vec<Texture> = [
            (config.battlefield.filename(), false),
            ("CmNumWin.pcx", false),
            ("CCellGrd.pcx", true),
            ("CCellShd.pcx", true),
        ]
        .into_iter()
        .map(|(filename, with_transparency)| {
            let surface = if with_transparency {
                rr.load_pcx_with_transparency(filename)
            } else {
                rr.load_pcx(filename)
            }?;

            let texture = surface.as_texture(tc)?;

            Ok(texture)
        })
        .collect::<Result<_, Box<dyn Error>>>()?;

        let textures: [Texture; 4] = textures.try_into().ok().unwrap();

        Ok(Self(textures))
    }

    pub fn get(&self, texture: StaticTexture) -> &Texture {
        &self.0[texture as usize]
    }
}
