use std::error::Error;

use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;

use formats::pcx::{Image, ImageData};

pub trait ImageT: Sized {
    fn from_raw(raw: Image) -> Result<Self, Box<dyn Error>>;
    fn into_surface(self) -> Surface<'static>;
}

pub struct StaticImage {
    surface: Surface<'static>,
}

pub struct PaletteImage {
    surface: Surface<'static>,
    colors: Box<[Color]>,
}

enum Either {
    Static(StaticImage),
    Palette(PaletteImage),
}

impl ImageT for StaticImage {
    fn from_raw(raw: Image) -> Result<Self, Box<dyn Error>> {
        let either = pcx_to_surface(raw)?;
        let image = match either {
            Either::Static(static_image) => static_image,
            Either::Palette(palette_image) => StaticImage {
                surface: palette_image.surface,
            },
        };
        Ok(image)
    }

    fn into_surface(self) -> Surface<'static> {
        self.surface
    }
}

impl ImageT for PaletteImage {
    fn from_raw(raw: Image) -> Result<Self, Box<dyn Error>> {
        let either = pcx_to_surface(raw)?;
        match either {
            Either::Static(_) => Err("Image is static".into()),
            Either::Palette(palette_image) => Ok(palette_image),
        }
    }

    fn into_surface(self) -> Surface<'static> {
        self.surface
    }
}

impl PaletteImage {
    pub fn apply_transparency(&mut self) -> Result<(), Box<dyn Error>> {
        let color_changes = [0, 32, 64, 128, 128];
        for (index, alpha) in color_changes.into_iter().enumerate() {
            self.colors[index] = Color::RGBA(0, 0, 0, alpha);
        }
        let palette = Palette::with_colors(&self.colors)?;

        self.surface.set_color_key(true, Color::RGB(0, 0, 0))?;
        self.surface.set_palette(&palette)?;

        Ok(())
    }
}

fn pcx_to_surface(image: Image) -> Result<Either, Box<dyn Error>> {
    let Image {
        width,
        height,
        data: mut image_data,
        ..
    } = image;

    let mut surface = {
        let (pixels, pitch, pixel_format) = match image_data {
            ImageData::RGB24(ref mut bytes) => (bytes, width * 3, PixelFormatEnum::BGR24),
            ImageData::Index8 { ref mut bytes, .. } => (bytes, width, PixelFormatEnum::Index8),
        };

        let surface = Surface::from_data(pixels, width, height, pitch, pixel_format)?;
        surface.convert_format(surface.pixel_format_enum())?
    };

    let result = match image_data {
        ImageData::RGB24(_) => Either::Static(StaticImage { surface }),
        ImageData::Index8 { colors, .. } => {
            let colors: Box<[Color]> = colors
                .iter()
                .map(|c| Color::RGB(c.red, c.green, c.blue))
                .collect();

            let palette = Palette::with_colors(&colors)?;
            surface.set_palette(&palette)?;

            Either::Palette(PaletteImage { surface, colors })
        }
    };

    Ok(result)
}
// if apply_transparency {
//     let color_changes = [0, 32, 64, 128, 128];

//     for (index, alpha) in color_changes.into_iter().enumerate() {
//         colors[index] = Color::RGBA(0, 0, 0, alpha);
//     }

//     surface.set_color_key(true, Color::RGB(0, 0, 0))?;
// }
