use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;

use formats::pcx;

use common::error::{AnyHow, AnyWay};

pub struct StaticImage {
    surface: Surface<'static>,
}

pub struct PaletteImage {
    surface: Surface<'static>,
    colors: Box<[Color]>,
}

enum Image {
    Static(StaticImage),
    Palette(PaletteImage),
}

impl StaticImage {
    pub fn from_bytes(bytes: Box<[u8]>) -> AnyHow<Self> {
        let image = Image::from_bytes(bytes)?;
        let static_image = match image {
            Image::Static(static_image) => static_image,
            Image::Palette(palette_image) => StaticImage {
                surface: palette_image.surface,
            },
        };
        Ok(static_image)
    }

    pub fn into_surface(self) -> Surface<'static> {
        self.surface
    }
}

impl PaletteImage {
    pub fn from_bytes(bytes: Box<[u8]>) -> AnyHow<Self> {
        let image = Image::from_bytes(bytes)?;
        match image {
            Image::Static(_) => Err("Image is static".into()),
            Image::Palette(palette_image) => Ok(palette_image),
        }
    }

    pub fn into_surface(self) -> Surface<'static> {
        self.surface
    }

    pub fn apply_transparency(&mut self) -> AnyWay {
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

impl Image {
    fn from_bytes(bytes: Box<[u8]>) -> AnyHow<Self> {
        let pcx = pcx::from_bytes(bytes)?;
        let image = pcx_to_surface(pcx)?;
        Ok(image)
    }
}

fn pcx_to_surface(image: pcx::Image) -> AnyHow<Image> {
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

    let result = match image_data {
        pcx::ImageData::RGB24(_) => Image::Static(StaticImage { surface }),
        pcx::ImageData::Index8 { colors, .. } => {
            let colors: Box<[Color]> = colors
                .iter()
                .map(|c| Color::RGB(c.red, c.green, c.blue))
                .collect();

            let palette = Palette::with_colors(&colors)?;
            surface.set_palette(&palette)?;

            Image::Palette(PaletteImage { surface, colors })
        }
    };

    Ok(result)
}
