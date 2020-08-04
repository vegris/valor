use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Deref;

extern crate sdl2;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, PixelFormatEnum};


pub struct DefSprite {
    pub size: u32,
    pub full_width: u32,
    pub full_height: u32,
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
    pub surface: Surface<'static>
}

pub struct DefContainer {
    pub type_: u32,
    pub colors: Box<[Color]>,
    pub names2sprites: HashMap<String, DefSprite>,
    pub blocks2names: HashMap<u32, Box<[String]>>
}

impl DefSprite {
    fn from_bytes(def_data: &[u8], offset: u32) -> Self {
        let data = &def_data[offset as usize..];
        let (header, image_data) = data.split_at(32);
        let [size, format, fw, fh, w, h, lm, tm]: [u32; 8] = header
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
                                    .map(u32::from_ne_bytes);
                for line_offset in line_offsets {
                    let mut row = &image_data[line_offset as usize..];
                    let mut total_row_length = 0;
                    while total_row_length < w {
                        let (code, length) = (row[0], row[1] as u32 + 1); // idk why we need to +1 length
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
                                    .map(u16::from_ne_bytes);
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
                    .map(u16::from_ne_bytes);
                
                for offset in line_offsets {
                    let mut row = &image_data[offset as usize..];
                    let mut total_block_length = 0;
                    while total_block_length < 32 {
                        let segment = row[0];
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

        let surface = Surface::from_data(&mut pixel_data, w, h, w * 1, PixelFormatEnum::Index8).unwrap();
        let static_surface = surface.convert_format(surface.pixel_format_enum()).unwrap();
        DefSprite {
            size,
            full_width: fw,
            full_height: fh,
            width: w,
            height: h,
            left_margin: lm,
            top_margin: tm,
            surface: static_surface
        }
    }
}

impl DefContainer {
    pub fn from_bytes(bytes: Box<[u8]>) -> Self {
        let (header, payload) = bytes.split_at(16);

        let type_ = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let n_blocks = u32::from_le_bytes(header[12..16].try_into().unwrap());

        let (palette_data, pixel_data) = payload.split_at(256 * 3);
        let colors = palette_data
            .chunks_exact(3)
            .map(|chunk| Color::RGB(chunk[0], chunk[1], chunk[2]))
            .collect::<Box<[Color]>>();
        
        let mut names2sprites: HashMap<String, DefSprite> = HashMap::new();
        let mut blocks2names: HashMap<u32, Box<[String]>> = HashMap::with_capacity(n_blocks as usize);
        (0..n_blocks).fold(pixel_data, |cur_data, _| {
            let (block_header, rest_data) = cur_data.split_at(16);
            let block_id = u32::from_ne_bytes(block_header[..4].try_into().unwrap());
            let n_entries = u32::from_ne_bytes(block_header[4..8].try_into().unwrap()) as usize;

            let (block_data, rest_data) = rest_data.split_at((13 + 4) * n_entries);
            let (names_buf, offsets_buf) = block_data.split_at(13 * n_entries);

            let names = names_buf
                        .chunks_exact(13)
                        .map(|chunk| chunk.split(|chr| *chr == 0).next().unwrap())
                        .map(|cut_bytes| String::from_utf8(cut_bytes.to_vec()).unwrap());
            let sprites = offsets_buf
                        .chunks_exact(4)
                        .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()))
                        .map(|offset| DefSprite::from_bytes(bytes.deref(), offset));

            let block = names.clone().collect::<Box<[String]>>();
            blocks2names.insert(block_id, block);

            names2sprites.extend(Iterator::zip(names, sprites));
            
            rest_data
            });

        Self {type_, colors, names2sprites, blocks2names}
    }
}