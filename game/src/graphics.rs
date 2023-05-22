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
        let graphics = Graphics {
            battlefield: rr.load_pcx(config.battlefield.filename())?.as_texture(tc)?,
            grid_cell: rr
                .load_pcx_with_transparency("CCellGrd.pcx")?
                .as_texture(tc)?,
            grid_cell_shadow: rr
                .load_pcx_with_transparency("CCellShd.pcx")?
                .as_texture(tc)?,
            stack_count_bg: rr.load_pcx("CmNumWin.pcx")?.as_texture(tc)?,

            cursors: Cursors::load(rr),
        };
        Ok(graphics)
    }
}
