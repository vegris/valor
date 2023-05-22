use std::error::Error;

use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::config::Config;
use crate::registry::ResourceRegistry;

pub mod creature;
pub mod cursors;

use cursors::Cursors;

pub struct Graphics<'a> {
    pub battlefield: Texture<'a>,
    pub grid_cell: Texture<'a>,
    pub grid_cell_shadow: Texture<'a>,
    pub stack_count_bg: Texture<'a>,

    pub cursors: Cursors,
}

impl<'a> Graphics<'a> {
    pub fn init(
        config: &Config,
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

        let graphics = Graphics {
            battlefield,
            grid_cell,
            grid_cell_shadow,
            stack_count_bg,

            cursors: Cursors::load(rr),
        };
        Ok(graphics)
    }
}
