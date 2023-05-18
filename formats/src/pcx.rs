use std::convert::TryInto;
use std::error::Error;
use std::ops::Deref;

use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;

struct ImageInfo {
    size: u32,
    width: u32,
    height: u32,
}

pub struct RGB24(Surface<'static>);

impl RGB24 {
    pub fn to_surface(self) -> Result<Surface<'static>, Box<dyn Error>> {
        Ok(self.0)
    }

    fn from_bytes(data: &mut [u8], info: ImageInfo) -> Result<Self, Box<dyn Error>> {
        let surface = Surface::from_data(
            data,
            info.width,
            info.height,
            info.width * 3,
            PixelFormatEnum::BGR24,
        )?;
        let static_surface = surface.convert_format(surface.pixel_format_enum())?;

        Ok(Self(static_surface))
    }
}

pub struct Index8 {
    surface: Surface<'static>,
    colors: Box<[Color]>,
}

impl Index8 {
    pub fn to_surface(mut self) -> Result<Surface<'static>, Box<dyn Error>> {
        let palette = Palette::with_colors(&self.colors)?;
        self.surface.set_palette(&palette)?;
        Ok(self.surface)
    }

    pub fn apply_transparency(&mut self) -> Result<(), Box<dyn Error>> {
        let color_changes = [0, 32, 64, 128, 128];

        for (index, alpha) in color_changes.into_iter().enumerate() {
            self.colors[index] = Color::RGBA(0, 0, 0, alpha);
        }

        self.surface.set_color_key(true, Color::RGB(0, 0, 0))?;

        Ok(())
    }

    fn from_bytes(data: &mut [u8], info: ImageInfo) -> Result<Self, Box<dyn Error>> {
        let (pixel_data, palette_data) = data.split_at_mut(info.size as usize);

        let colors = palette_data
            .chunks_exact(3)
            .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
            .collect::<Box<[Color]>>();

        let surface = Surface::from_data(
            pixel_data,
            info.width,
            info.height,
            info.width,
            PixelFormatEnum::Index8,
        )?;

        let static_surface = surface.convert_format(surface.pixel_format_enum())?;

        Ok(Self {
            surface: static_surface,
            colors,
        })
    }
}

pub fn from_bytes(bytes: &mut [u8]) -> Result<either::Either<RGB24, Index8>, Box<dyn Error>> {
    let (header, data) = bytes.split_at_mut(12);
    let [size, width, height]: [u32; 3] = header
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .map(u32::from_ne_bytes)
        .collect::<Box<[u32]>>()
        .deref()
        .try_into()?;

    let info = ImageInfo {
        size,
        width,
        height,
    };

    let image = match size {
        size if size == width * height * 3 => either::Left(RGB24::from_bytes(data, info)?),
        size if size == width * height => either::Right(Index8::from_bytes(data, info)?),
        _ => return Err("Unknown pcx format!".into()),
    };

    Ok(image)
}
