use std::error::Error;

use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::sdl::Context;
use crate::{Config, ResourceRegistry};

use super::cursors::Cursors;

pub struct Statics<'a> {
    pub(super) battlefield: Texture<'a>,
    pub(super) grid_cell: Texture<'a>,
    pub(super) grid_cell_shadow: Texture<'a>,
    pub(super) stack_count_bg: Texture<'a>,

    pub(super) cursors: Cursors,

    pub(super) font: Font<'a, 'static>,
}

impl<'a> Statics<'a> {
    pub fn init(
        config: &Config,
        sdl_context: &'a Context,
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let [battlefield, grid_cell, grid_cell_shadow, stack_count_bg]: [Texture; 4] = [
            (config.battlefield.filename(), false),
            ("CCellGrd.pcx", true),
            ("CCellShd.pcx", true),
            ("CmNumWin.pcx", false),
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
        .collect::<Result<Vec<Texture>, Box<dyn Error>>>()?
        .try_into()
        .ok()
        .unwrap();

        let font = sdl_context.load_font("/usr/share/fonts/TTF/OpenSans-Bold.ttf", 16)?;

        let graphics = Self {
            battlefield,
            grid_cell,
            grid_cell_shadow,
            stack_count_bg,

            cursors: Cursors::load(rr),

            font,
        };
        Ok(graphics)
    }
}
