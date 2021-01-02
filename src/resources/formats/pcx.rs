use std::convert::TryInto;
use std::ops::Deref;

use crate::util::AnyError;

extern crate either;
use either::{Either, Left, Right};

extern crate sdl2;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, Palette, PixelFormatEnum};

pub struct PcxImage(Either<RGB24PCX, Index8PCX>);

struct RGB24PCX(Surface<'static>);
struct Index8PCX {
    surface: Surface<'static>,
    colors: Box<[Color]>
}

impl PcxImage {
    pub fn from_bytes(mut bytes: Box<[u8]>) -> Result<Self, AnyError> {
        let (header, data) = bytes.split_at_mut(12); 
        let [size, width, height]: [u32; 3] = header
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .map(u32::from_ne_bytes)
            .collect::<Box<[u32]>>()
            .deref()
            .try_into()?;
        
        let image = 
            if size == width * height * 3 {
                let surface = Surface::from_data(data, width, height, width * 3, PixelFormatEnum::BGR24)?;
                let static_surface = surface.convert_format(surface.pixel_format_enum())?;
                Left(RGB24PCX(static_surface))
            } 
            else if size == width * height { 
                let (pixel_data, palette_data) = data.split_at_mut(size as usize);

                let colors = palette_data
                    .chunks_exact(3)
                    .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
                    .collect::<Box<[Color]>>();
                
                let surface = Surface::from_data(pixel_data, width, height, width * 1, PixelFormatEnum::Index8)?;
                let static_surface = surface.convert_format(surface.pixel_format_enum())?;
                Right(Index8PCX {surface: static_surface, colors})
            }
            else {
                panic!("Unknown pcx format!")
            };
        Ok(Self(image))
    }

    pub fn apply_transparency(&mut self) -> Result<(), String> {
        self.0
            .as_mut()
            .right()
            .ok_or("Transparency can only be applied to PCX with palette".to_string())
            .and_then(|Index8PCX {surface, colors}| {
                colors[0] = Color::RGBA(0, 0, 0, 0);
                colors[1] = Color::RGBA(0, 0, 0, 32);
                colors[2] = Color::RGBA(0, 0, 0, 64);
                colors[3] = Color::RGBA(0, 0, 0, 128);
                colors[4] = Color::RGBA(0, 0, 0, 128);
                surface.set_color_key(true, Color::RGB(0, 0, 0))
            })
    }

    pub fn to_surface(self) -> Result<Surface<'static>, AnyError> {
        match self.0 {
            Left(RGB24PCX(surface)) => Ok(surface),
            Right(Index8PCX {mut surface, colors}) =>  {
                let palette = Palette::with_colors(&colors)?;
                surface.set_palette(&palette)?;
                Ok(surface)
            }
        }
    }
}