use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;

use formats::pcx;

use crate::error::{AnyHow, AnyWay};

pub trait ImageT: Sized + TryFrom<Image, Error = &'static str> + Into<Surface<'static>> {}

pub struct StaticImage {
    surface: Surface<'static>,
}

pub struct PaletteImage {
    surface: Surface<'static>,
    colors: Box<[Color]>,
}

pub enum Image {
    Static(StaticImage),
    Palette(PaletteImage),
}

pub fn from_bytes<Image: ImageT>(bytes: Box<[u8]>) -> AnyHow<Image> {
    let raw = pcx::from_bytes(bytes)?;
    let either = pcx_to_surface(raw)?;
    let image = either.try_into()?;
    Ok(image)
}

impl TryFrom<Image> for StaticImage {
    type Error = &'static str;
    fn try_from(value: Image) -> Result<Self, Self::Error> {
        let image = match value {
            Image::Static(static_image) => static_image,
            Image::Palette(palette_image) => StaticImage {
                surface: palette_image.surface,
            },
        };
        Ok(image)
    }
}

impl From<StaticImage> for Surface<'static> {
    fn from(value: StaticImage) -> Self {
        value.surface
    }
}

impl ImageT for StaticImage {}

impl TryFrom<Image> for PaletteImage {
    type Error = &'static str;
    fn try_from(value: Image) -> Result<Self, Self::Error> {
        match value {
            Image::Static(_) => Err("Image is static"),
            Image::Palette(palette_image) => Ok(palette_image),
        }
    }
}

impl From<PaletteImage> for Surface<'static> {
    fn from(value: PaletteImage) -> Self {
        value.surface
    }
}

impl ImageT for PaletteImage {}

impl PaletteImage {
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
