use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Deref;

extern crate sdl2;
use sdl2::pixels::{Color, Palette};

pub struct DefIndex {
    pub type_: u32,
    pub palette: Palette,
    pub sprites_index: HashMap<String, PixelData>,
    pub blocks: HashMap<u32, Box<[String]>>
}

pub struct PixelData {
    pub width: u32,
    pub height: u32,
    pub pixels: Box<[u8]>
}

pub fn parse(def_data: &[u8]) -> DefIndex {
    let (header, payload) = def_data.split_at(16);

    let type_ = u32::from_le_bytes(header[0..4].try_into().unwrap());
    let n_blocks = u32::from_le_bytes(header[12..16].try_into().unwrap());

    let (palette_data, mut pixel_data) = payload.split_at(256 * 3);
    let colors = palette_data
        .chunks_exact(3)
        .map(|chunk| Color::RGB(chunk[0], chunk[1], chunk[2]))
        .collect::<Box<[Color]>>();
    let palette = Palette::with_colors(&colors).unwrap();
    
    let mut sprites_index: HashMap<String, PixelData> = HashMap::new();
    let mut blocks: HashMap<u32, Box<[String]>> = HashMap::with_capacity(n_blocks as usize);
    // Wow thats ugly! Should probably use some folds instead
    for _ in 0..n_blocks {
        let (block_header, other) = pixel_data.split_at(16);
        let block_id = u32::from_le_bytes(block_header[..4].try_into().unwrap());
        let n_entries = u32::from_le_bytes(block_header[4..8].try_into().unwrap()) as usize;

        let (block_data, other) = other.split_at((13 + 4) * n_entries);
        let (names_buf, offsets_buf) = block_data.split_at(13 * n_entries);

        let names = names_buf
                    .chunks_exact(13)
                    .map(|bytes| bytes.split(|chr| *chr == 0).next().unwrap())
                    .map(|cut_bytes| String::from_utf8(cut_bytes.to_vec()).unwrap());
        let sprites = offsets_buf
                    .chunks_exact(4)
                    .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()))
                    .map(|offset| extract_pixel_data(def_data, offset));

        let block = names.clone().collect::<Box<[String]>>();
        blocks.insert(block_id, block);

        sprites_index.extend(Iterator::zip(names, sprites));
        
        pixel_data = other;
    }
    DefIndex {type_, palette, sprites_index, blocks}
}

fn extract_pixel_data(def_data: &[u8], offset: u32) -> PixelData {
    let data = &def_data[offset as usize..];
    let (header, image_data) = data.split_at(32);
    let [_size, format, fw, fh, w, h, lm, tm]: [u32; 8] = header
        .chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(u32::from_le_bytes)
        .collect::<Box<[u32]>>()
        .deref().try_into().unwrap();

    let mut pixel_data: Vec<u8> = Vec::with_capacity((w * h) as usize);
    match format {
        0 => pixel_data.extend_from_slice(&image_data[..(w * h) as usize]),
        1 => {
            let line_offsets = image_data[..(4 * h) as usize]
                                .chunks_exact(4)
                                .map(|chunk| chunk.try_into().unwrap())
                                .map(u32::from_le_bytes);
            for line_offset in line_offsets {
                let mut row = &image_data[line_offset as usize..];
                let mut total_row_length = 0;
                while total_row_length < w {
                    let (code, length) = (u8::from_le(row[0]), u8::from_le(row[1]) as u32 + 1); // idk why we need to +1 length
                    match code {
                        0xff => {
                            pixel_data.extend_from_slice(&row[2..length as usize + 2]);
                            row = &row[length as usize + 2..];
                        }
                        _ => {
                                for _ in 0..length { pixel_data.push(code) }; 
                                row = &row[2..];
                        }
                    }
                    total_row_length += length;
                }
            }
        },
        2 => {
            let line_offsets = image_data[..(2 * h) as usize]
                                .chunks_exact(2)
                                .map(|chunk| chunk.try_into().unwrap())
                                .map(u16::from_le_bytes);
            for line_offset in line_offsets {
                let mut row = &image_data[line_offset as usize..];
                let mut total_row_length = 0;
                while total_row_length < w {
                    let segment = u8::from_le(row[0]);
                    let code = segment >> 5;
                    let length = (segment & 0x1f) + 1;
                    match code {
                        7 => {
                            pixel_data.extend_from_slice(&row[1..length as usize + 1]);
                            row = &row[length as usize + 1..];
                        },
                        _ => {
                            for _ in 0..length { pixel_data.push(code) };
                            row = &row[1..];
                        }
                    }
                    total_row_length += length as u32;
                }
            }
        },
        3 => {
            // each row is split into 32 byte long blocks which are individually encoded
            // two bytes store the offset for each block per line
            let line_offsets = image_data
                .chunks_exact(2)
                .take((h * w / 32) as usize)
                .map(|chunk| chunk.try_into().unwrap())
                .map(u16::from_le_bytes);
            
            for offset in line_offsets {
                let mut row = &image_data[offset as usize..];
                let mut total_block_length = 0;
                while total_block_length < 32 {
                    let segment = u8::from_le(row[0]);
                    let code = segment >> 5;
                    let length = (segment & 0x1f) + 1;
                    match code {
                        7 => {
                            pixel_data.extend_from_slice(&row[1..length as usize + 1]);
                            row = &row[length as usize + 1..];
                        },
                        _ => {
                            for _ in 0..length { pixel_data.push(code) };
                            row = &row[1..];
                        }
                    }
                    total_block_length += length as u32;
                }
            }
        }
        _ => panic!("Unknown format!")
    }
    PixelData{ width: w, height: h, pixels: pixel_data.into_boxed_slice()}
}
