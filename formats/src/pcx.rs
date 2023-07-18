use std::convert::TryInto;
use std::error::Error;
use std::ops::Deref;

pub struct Image {
    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub data: ImageData,
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub enum ImageData {
    RGB24(Box<[u8]>),
    Index8 {
        bytes: Box<[u8]>,
        colors: Box<[Color]>,
    },
}

pub fn from_bytes(bytes: &mut [u8]) -> Result<Image, Box<dyn Error>> {
    let (header, data) = bytes.split_at_mut(12);
    let [size, width, height]: [u32; 3] = header
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .map(u32::from_ne_bytes)
        .collect::<Box<[u32]>>()
        .deref()
        .try_into()?;

    let image_data = image_data(size, width, height, data)?;

    Ok(Image {
        size,
        width,
        height,
        data: image_data,
    })
}

fn image_data(size: u32, width: u32, height: u32, bytes: &[u8]) -> Result<ImageData, String> {
    match size {
        size if size == width * height * 3 => Ok(ImageData::RGB24(bytes.into())),
        size if size == width * height => {
            let (pixel_data, palette_data) = bytes.split_at(size as usize);

            let colors = palette_data
                .chunks_exact(3)
                .map(|chunk| {
                    let [red, green, blue]: [u8; 3] = chunk.try_into().unwrap();
                    Color { red, green, blue }
                })
                .collect::<Box<_>>();

            let image_data = ImageData::Index8 {
                bytes: pixel_data.into(),
                colors,
            };
            Ok(image_data)
        }
        _ => Err("Unknown pcx format!".into()),
    }
}
