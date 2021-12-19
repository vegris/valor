mod header;
mod format;
mod parser;

use header::Header;
use parser::parse_pixel_data;

extern crate sdl2;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;

pub struct Sprite {
    pub full_size: u32,
    pub full_width: u32,
    pub full_height: u32,
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
    pub surface: Surface<'static>
}

impl Sprite {
    pub fn from_bytes(def_data: &[u8], offset: u32) -> Self {
        let data = &def_data[offset as usize..];
        let (header_data, image_data) = data.split_at(Header::LENGTH);

        let header = Header::parse(header_data);

        let pixel_data = parse_pixel_data(header, image_data);
        let surface = create_surface(header, pixel_data);

        Self::new(header, surface)
    }

    fn new(header: Header, surface: Surface<'static>) -> Self {
        let Header {
            full_size, format: _,
            full_width, full_height,
            width, height,
            left_margin, top_margin,
        } = header;

        Self {
            full_size,
            full_width,
            full_height,
            width,
            height,
            left_margin,
            top_margin,
            surface
        }
    }
}

fn create_surface(header: Header, mut pixel_data: Box<[u8]>) -> Surface<'static> {
    let Header{ width, height, .. } = header;

    let surface = Surface::from_data(&mut pixel_data, width, height, width * 1, PixelFormatEnum::Index8).unwrap();
    let static_surface = surface.convert_format(surface.pixel_format_enum()).unwrap();

    static_surface
}
