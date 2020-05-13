use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::SeekFrom;
use std::convert::TryInto;
use std::ops::Deref;

extern crate flate2;
use flate2::read::ZlibDecoder;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::pixels::{PixelFormatEnum, Color, Palette};

const LOD_ARCHIVE: &'static str = "/home/vsevolod/Wine/HoMM3/drive_c/HoMM3/Data/H3sprite.lod";

#[derive(Debug)]
struct FileInfo {
    offset: u32,
    size: u32,
    compressed: bool
}

struct LodIndex {
    handle: File,
    registry: HashMap<String, FileInfo>
}

impl LodIndex {
    fn open(path: &'static str) -> Self {
        let mut f = File::open(path).unwrap();
        let mut parse_buffer: [u8; 16] = [0; 16];

        f.seek(SeekFrom::Start(8)).unwrap();

        f.read_exact(&mut parse_buffer).unwrap();
        let total_files = u32::from_le_bytes(parse_buffer[0..4].try_into().unwrap());

        f.seek(SeekFrom::Start(92)).unwrap();
        let mut file_registry: HashMap<String, FileInfo> = HashMap::with_capacity(total_files as usize);

        for _ in 0..total_files {
            // Reading filename
            f.read_exact(&mut parse_buffer).unwrap();
            let str_bytes = parse_buffer.split(|chr| *chr == 0).next().unwrap();
            let filename = String::from_utf8(str_bytes.to_vec()).unwrap();
            
            // Reading sizes and offset
            f.read_exact(&mut parse_buffer).unwrap();
            let offset = u32::from_le_bytes(parse_buffer[0..4].try_into().unwrap());
            let size = u32::from_le_bytes(parse_buffer[4..8].try_into().unwrap());
            let csize = u32::from_le_bytes(parse_buffer[12..16].try_into().unwrap());

            let file_info = FileInfo { offset, size, compressed: csize != 0 };
            file_registry.insert(filename, file_info);
        }
    
        LodIndex {handle: f, registry: file_registry}
    }

    fn read_file(&mut self, filename: &String) -> Vec<u8> {
        let FileInfo { offset, size, compressed } = *self.registry.get(filename).unwrap();
        self.handle.seek(SeekFrom::Start(offset as u64)).unwrap();

        let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);
        buffer.resize(size as usize, 0);
        if compressed {
            ZlibDecoder::new(&mut self.handle).read_exact(buffer.as_mut_slice()).unwrap();
        }
        else {
            self.handle.read_exact(buffer.as_mut_slice()).unwrap();
        }
        buffer
    }
}

#[derive(Debug)]
struct DefIndex {
    data: Vec<u8>,
    type_: u32,
    palette: Vec<Color>,
    registry: HashMap<String, u32>,
    blocks: HashMap<u32, Vec<String>>
}

impl DefIndex {
    fn open(lod_index: &mut LodIndex, filename: &String) -> Self {
        let data = lod_index.read_file(filename);
        let (header, payload) = &data.split_at(16);

        let type_ = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let n_blocks = u32::from_le_bytes(header[12..16].try_into().unwrap());

        let (palette_slice, mut cur_data) = payload.split_at(256*3);

        let palette = palette_slice.chunks_exact(3)
                             .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
                             .collect::<Vec<Color>>();

        let mut registry: HashMap<String, u32> = HashMap::new();
        let mut blocks: HashMap<u32, Vec<String>> = HashMap::with_capacity(n_blocks as usize);
        // Wow thats ugly! Should probably use some folds instead
        for _ in 0..n_blocks {
            let (block_header, other) = cur_data.split_at(16);
            let block_id = u32::from_le_bytes(block_header[..4].try_into().unwrap());
            let n_entries = u32::from_le_bytes(block_header[4..8].try_into().unwrap()) as usize;

            let (block_data, other) = other.split_at((13 + 4) * n_entries);
            let (names_buf, offsets_buf) = block_data.split_at(13 * n_entries);
            let names = names_buf
                        .chunks_exact(13)
                        .map(|bytes| bytes.split(|chr| *chr == 0).next().unwrap())
                        .map(|cut_bytes| String::from_utf8(cut_bytes.to_vec()).unwrap());
            
            let offsets = offsets_buf
                          .chunks_exact(4)
                          .map(|bytes| u32::from_le_bytes(bytes.try_into().unwrap()));

            let mut block: Vec<String> = Vec::with_capacity(n_entries as usize);
            for (name, offset) in Iterator::zip(names, offsets) {
                block.push(name.clone());
                registry.insert(name, offset);
            }
            blocks.insert(block_id, block);
            cur_data = other;
        }
        dbg!(&type_);
        dbg!(&registry);
        dbg!(&blocks);
        DefIndex {data, type_, palette, registry, blocks}
    }

    fn load_image_data(&self, name: &String) -> (u32, u32, Vec<u8>) {
        let offset = *self.registry.get(name).unwrap();
        let data = &self.data.as_slice()[offset as usize..];
        let (header, image_data) = data.split_at(32);
        let [_size, format, fw, fh, w, h, lm, tm]: [u32; 8] = header
            .chunks_exact(4)
            .map(TryInto::try_into)
            .map(Result::unwrap)
            .map(u32::from_le_bytes)
            .collect::<Box<[u32]>>()
            .deref().try_into().unwrap();

        dbg!(format, fw, fh, w, h, lm, tm);

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
        (w, h, pixel_data)
    }
}

fn surface_from_data(data: &mut Vec<u8>) -> Surface {
    // ЭТО РАЗМЕР КАРТИНКИ, А НЕ БУФЕРА data
    let size = u32::from_le_bytes(data[..4].try_into().unwrap());
    let width = u32::from_le_bytes(data[4..8].try_into().unwrap());
    let height = u32::from_le_bytes(data[8..12].try_into().unwrap());

    if size == width * height * 3 {
        println!("Ordinary!");
        Surface::from_data(&mut data[12..], width, height, 3*width, PixelFormatEnum::BGR24).unwrap()
    }
    else {
        println!("With palette!");
        let colors = data[12 + size as usize .. 12 + size as usize + 256 * 3]
                        .chunks_exact(3)
                        .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
                        .collect::<Box<[Color]>>();
        
        let palette = Palette::with_colors(&colors).unwrap();

        let mut surface = Surface::from_data(&mut data[12..], width, height, 1 * width, PixelFormatEnum::Index8).unwrap();
        surface.set_palette(&palette).unwrap();
        surface
    }
}

fn main() {
    let mut manager = LodIndex::open(LOD_ARCHIVE);

    let def_index = DefIndex::open(&mut manager, &String::from("AB01_.def"));
    let (width, height, mut pixel_data) = def_index.load_image_data(&String::from("ab01_09.pcx"));

    // let mut contents = manager.read_file(&String::from("CBONE2A2.PCX"));

    let sdl_context = sdl2::init().unwrap(); 
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let mut surface = Surface::from_data(pixel_data.as_mut_slice(), width, height, 1 * width, PixelFormatEnum::Index8).unwrap();
    // let surface = surface_from_data(&mut contents);
    let palette = Palette::with_colors(def_index.palette.as_slice()).unwrap();
    surface.set_palette(&palette).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { 
                    break 'running 
                },
                _ => {}
            }
        }
        canvas.clear();
        canvas.copy(&texture, None, surface.rect()).unwrap();
        canvas.present();
    }
}
