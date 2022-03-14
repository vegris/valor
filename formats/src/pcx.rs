use std::convert::TryInto;
use std::ops::Deref;
use std::error::Error;

extern crate sdl2;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, Palette, PixelFormatEnum};

pub enum PcxImage {
    RGB24PCX(Surface<'static>),
    Index8PCX {
        surface: Surface<'static>,
        colors: Box<[Color]>
    }
}

impl PcxImage {
    pub fn from_bytes(bytes: &mut [u8]) -> Result<Self, Box<dyn Error>> {
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
                
                Self::RGB24PCX(static_surface)
            } 
            else if size == width * height { 
                let (pixel_data, palette_data) = data.split_at_mut(size as usize);

                let colors = palette_data
                    .chunks_exact(3)
                    .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
                    .collect::<Box<[Color]>>();
                
                let surface = Surface::from_data(pixel_data, width, height, width, PixelFormatEnum::Index8)?;
                let static_surface = surface.convert_format(surface.pixel_format_enum())?;
                Self::Index8PCX {surface: static_surface, colors}
            }
            else {
                panic!("Unknown pcx format!")
            };
        Ok(image)
    }

    pub fn apply_transparency(&mut self) {
        if let Self::Index8PCX {surface, colors} = self {
            colors[0] = Color::RGBA(0, 0, 0, 0);
            colors[1] = Color::RGBA(0, 0, 0, 32);
            colors[2] = Color::RGBA(0, 0, 0, 64);
            colors[3] = Color::RGBA(0, 0, 0, 128);
            colors[4] = Color::RGBA(0, 0, 0, 128);
            surface.set_color_key(true, Color::RGB(0, 0, 0)).unwrap()
        } else {
            panic!("Transparency can only be applied to PCX with palette")
        }
    }

    pub fn to_surface(self) -> Result<Surface<'static>, Box<dyn Error>> {
        match self {
            Self::RGB24PCX(surface) => Ok(surface),
            Self::Index8PCX {mut surface, colors} =>  {
                let palette = Palette::with_colors(&colors)?;
                surface.set_palette(&palette)?;
                Ok(surface)
            }
        }
    }
}