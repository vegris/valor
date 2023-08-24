use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Deref;

use crate::Color;

mod sprite;
pub use sprite::Sprite;

pub struct Container {
    pub type_: u32,
    pub colors: Box<[Color]>,
    pub names2sprites: HashMap<String, Sprite>,
    pub blocks2names: HashMap<u32, Box<[String]>>,
}

impl Container {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let (header, payload) = bytes.split_at(16);

        let type_ = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let n_blocks = u32::from_le_bytes(header[12..16].try_into().unwrap());

        let (palette_data, pixel_data) = payload.split_at(256 * 3);
        let colors = palette_data
            .chunks_exact(3)
            .map(|chunk| Color {
                red: chunk[0],
                green: chunk[1],
                blue: chunk[2],
            })
            .collect::<Box<[Color]>>();

        let mut names2sprites: HashMap<String, Sprite> = HashMap::new();
        let mut blocks2names: HashMap<u32, Box<[String]>> =
            HashMap::with_capacity(n_blocks as usize);
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
                .map(|offset| Sprite::from_bytes(bytes.deref(), offset));

            let block = names.clone().collect::<Box<[String]>>();
            blocks2names.insert(block_id, block);

            names2sprites.extend(Iterator::zip(names, sprites));

            rest_data
        });

        Self {
            type_,
            colors,
            names2sprites,
            blocks2names,
        }
    }
}
