use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

use crate::error::AnyHow;
use crate::registry::spritesheets::{ContainerType, GroupIndex, SpriteGroup, SpriteGroupType};
use crate::{Config, ResourceRegistry};

use super::cursors::Cursors;
use super::spritesheet::hero::AnimationType;
use super::spritesheet::Spritesheet;

pub struct Statics<'a> {
    pub cursors: Cursors,
    pub font: Font<'a, 'static>,
    pub textures: Textures<'a>,
    pub heroes: [Option<Spritesheet<AnimationType>>; 2],
    pub ui: UI,
}

impl<'a> Statics<'a> {
    pub fn init(
        config: &Config,
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
        ttf_context: &'a sdl2::ttf::Sdl2TtfContext,
    ) -> AnyHow<Self> {
        let font_path = "/usr/share/fonts/TTF/OpenSans-Bold.ttf";
        let font_size = 16;

        let heroes = config.armies.map(|army| {
            army.hero.map(|h| {
                let hero_def = rr.load_def(h.class().spritesheet_filename());

                Spritesheet::from_def(hero_def)
            })
        });

        Ok(Self {
            cursors: Cursors::load(rr),
            font: ttf_context.load_font(font_path, font_size)?,
            textures: Textures::load(config, rr, tc)?,
            heroes,
            ui: UI::load(rr),
        })
    }
}

#[derive(Clone, Copy, EnumCount)]
pub enum StaticTexture {
    Battlefield,
    MenuBackground,
    StackCountBackground,
    GridCell,
    GridCellShadow,
}

pub struct Textures<'a>([Texture<'a>; StaticTexture::COUNT]);

impl<'a> Textures<'a> {
    fn load(
        config: &Config,
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
    ) -> AnyHow<Self> {
        let textures: Vec<Texture> = [
            (config.battlefield.filename(), false),
            ("cbar.pcx", false),
            ("CmNumWin.pcx", false),
            ("CCellGrd.pcx", true),
            ("CCellShd.pcx", true),
        ]
        .into_iter()
        .map(|(filename, with_transparency)| {
            let surface = if with_transparency {
                let mut image = rr.load_palette_image(filename)?;
                image.apply_transparency()?;
                image.into_surface()
            } else {
                let image = rr.load_static_image(filename)?;
                image.into_surface()
            };

            let texture = surface.as_texture(tc)?;

            Ok(texture)
        })
        .collect::<AnyHow<_>>()?;

        let textures: [Texture; StaticTexture::COUNT] = textures.try_into().ok().unwrap();

        Ok(Self(textures))
    }

    pub fn get(&self, texture: StaticTexture) -> &Texture {
        &self.0[texture as usize]
    }
}

#[derive(EnumCount, EnumIter)]
pub enum Buttons {
    Surrender,
    Retreat,
    Settings,
    AutoBattle,
    BookOfMagic,
    Wait,
    Defend,
}

impl Buttons {
    fn filename(self) -> &'static str {
        match self {
            Self::Surrender => "icm001.def",
            Self::Retreat => "icm002.def",
            Self::Settings => "icm003.def",
            Self::AutoBattle => "icm004.def",
            Self::BookOfMagic => "icm005.def",
            Self::Wait => "icm006.def",
            Self::Defend => "icm007.def",
        }
    }
}

#[allow(unused)]
#[derive(Clone, Copy, EnumCount)]
pub enum ButtonState {
    Base,
    Pressed,
    Disabled,
    Hovered,
}

impl ContainerType for ButtonState {
    const CONTAINER_TYPE: u32 = 71;
}

impl GroupIndex for ButtonState {
    fn index(&self) -> usize {
        *self as usize
    }
}

impl SpriteGroupType for ButtonState {}

pub struct UI([SpriteGroup<ButtonState>; Buttons::COUNT]);

impl UI {
    pub fn get(&self, button: Buttons) -> &SpriteGroup<ButtonState> {
        &self.0[button as usize]
    }

    fn load(rr: &mut ResourceRegistry) -> Self {
        let buttons = Buttons::iter()
            .map(|b| rr.load_sprite_group(b.filename()))
            .collect::<Vec<_>>()
            .try_into()
            .ok()
            .unwrap();

        Self(buttons)
    }
}
