use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;

use formats::pcx;
use formats::pcx::{Image, ImageData};

pub trait ImageT:
    Sized + TryFrom<inner::Either, Error = &'static str> + Into<Surface<'static>>
{
}

type Error = Box<dyn std::error::Error>;

pub struct StaticImage {
    surface: Surface<'static>,
}

pub struct PaletteImage {
    surface: Surface<'static>,
    colors: Box<[Color]>,
}

// Silence 'private in public' warning
mod inner {
    pub enum Either {
        Static(super::StaticImage),
        Palette(super::PaletteImage),
    }
}
use inner::Either;

pub fn from_bytes<Image: ImageT>(bytes: Box<[u8]>) -> Result<Image, Error> {
    let raw = pcx::from_bytes(bytes)?;
    let either = pcx_to_surface(raw)?;
    let image = either.try_into()?;
    Ok(image)
}

impl TryFrom<Either> for StaticImage {
    type Error = &'static str;
    fn try_from(value: Either) -> Result<Self, Self::Error> {
        let image = match value {
            Either::Static(static_image) => static_image,
            Either::Palette(palette_image) => StaticImage {
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

impl TryFrom<Either> for PaletteImage {
    type Error = &'static str;
    fn try_from(value: Either) -> Result<Self, Self::Error> {
        match value {
            Either::Static(_) => Err("Image is static"),
            Either::Palette(palette_image) => Ok(palette_image),
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
    pub fn apply_transparency(&mut self) -> Result<(), Error> {
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

fn pcx_to_surface(image: Image) -> Result<Either, Error> {
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
