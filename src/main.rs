use std::io::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::SeekFrom;
use std::convert::TryInto;

extern crate flate2;
use flate2::read::ZlibDecoder;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2::pixels::{PixelFormatEnum, Color, Palette};

const LOD_ARCHIVE: &'static str = "/home/vsevolod/Wine/HoMM3/drive_c/HoMM3/Data/H3bitmap.lod";

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

fn surface_from_data(data: &mut Vec<u8>) -> Surface{
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
                        .chunks(3)
                        .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
                        .collect::<Vec<Color>>();
        
        let palette = Palette::with_colors(&colors).unwrap();

        let mut surface = Surface::from_data(&mut data[12..], width, height, 1 * width, PixelFormatEnum::Index8).unwrap();
        surface.set_palette(&palette).unwrap();
        surface
    }
}

fn main() {
    let mut manager = LodIndex::open(LOD_ARCHIVE);

    let mut contents = manager.read_file(&String::from("CBONE2A2.PCX"));


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

    let surface = surface_from_data(&mut contents);
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
