use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::Deref;

extern crate sdl2;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::surface::Surface;
use sdl2::pixels::{Color, PixelFormatEnum, Palette};

use super::lod_index::LodIndex;
use super::def_parser;
use super::def_parser::DefIndex;

pub struct Animation<'a> {
    pub frames: HashMap<String, Texture<'a>>,
    blocks: HashMap<u32, Box<[String]>>
}

pub struct ResourceRegistry {
    texture_creator: TextureCreator<WindowContext>,
    resource_handles: HashMap<String, LodIndex>,
}

impl ResourceRegistry {
    pub fn new(canvas: &WindowCanvas, resource_files: &[&str]) -> Self {
        let texture_creator = canvas.texture_creator();

        let resource_handles = resource_files
            .iter()
            .map(|path| ((*path).to_owned(), LodIndex::open(path)))
            .collect::<HashMap<String, LodIndex>>();
        
        ResourceRegistry {
            texture_creator,
            resource_handles
        }
    }

    pub fn get_texture(&mut self, archive: &str, file: &str) -> Texture {
        let lod_handle = self.resource_handles.get_mut(archive).unwrap();
        let pcx_data = lod_handle.read_file(file);
        pcx_data_to_texture(&self.texture_creator, pcx_data.deref())
    }

    pub fn get_animation(&mut self, archive: &str, file: &str) -> Animation {
        let lod_handle = self.resource_handles.get_mut(archive).unwrap();
        let def_data = lod_handle.read_file(file);
        let def_index = def_parser::parse(&def_data);
        def_index_to_animation(&self.texture_creator, def_index)
    }
}

fn pcx_data_to_texture<'a>(tc: &'a TextureCreator<WindowContext>, pcx_data: &[u8]) -> Texture<'a> {
    let (header, data) = pcx_data.split_at(12);

    let [size, width, height]: [u32; 3] = header
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .map(u32::from_ne_bytes)
        .collect::<Box<[u32]>>()
        .deref()
        .try_into()
        .unwrap();
    
    let mut data = Vec::from(data);
    
    let surface = 
        if size == width * height * 3 {
            Surface::from_data(&mut data, width, height, 3 * width, PixelFormatEnum::BGR24).unwrap()
        } 
        else {
            let size = size as usize;
            let colors = data[size .. size + 256 * 3]
                .chunks_exact(3)
                .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
                .collect::<Box<[Color]>>();
            let palette = Palette::with_colors(&colors).unwrap();

            let mut surface = Surface::from_data(&mut data, width, height, 1 * width, PixelFormatEnum::Index8).unwrap();
            surface.set_palette(&palette).unwrap();
            surface
        };
    
    tc.create_texture_from_surface(surface).unwrap()
}

fn def_index_to_animation<'a>(tc: &'a TextureCreator<WindowContext>, def_index: DefIndex) -> Animation<'a> {
    let mut frames = HashMap::new();
    for (name, mut pix_data) in def_index.sprites_index {
        let mut surface = Surface::from_data(&mut pix_data.pixels, pix_data.width, pix_data.height, 1 * pix_data.width, PixelFormatEnum::Index8).unwrap();
        surface.set_palette(&def_index.palette).unwrap();
        let frame = tc.create_texture_from_surface(surface).unwrap();
        frames.insert(name, frame);
    }
    Animation{ frames, blocks: def_index.blocks }
}