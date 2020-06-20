use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::convert::TryInto;
use std::ops::Deref;

extern crate sdl2;
use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::surface::Surface;
use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;

extern crate flate2;
use flate2::read::ZlibDecoder;


const RESOURCES_ROOT: &str = "/home/vsevolod/Wine/HoMM3/drive_c/HoMM3/Data";
const PCX_ARCHIVE: &str = "H3bitmap.lod";
const DEF_ARCHIVE: &str = "H3sprite.lod";


struct LodFileInfo {
    offset: u32,
    size: u32,
    compressed: bool
}

struct LodIndex {
    handle: File,
    registry: HashMap<String, LodFileInfo>
}


pub struct ResourceRegistry {
    pcx_archive: LodIndex,
    def_archive: LodIndex
}


pub struct RawPcx {
    size: u32,
    width: u32,
    height: u32,
    pixel_data: Box<[u8]>
}

struct RawDefSprite {
    size: u32,
    full_width: u32,
    full_height: u32,
    width: u32,
    height: u32,
    left_margin: u32,
    top_margin: u32,
    pixel_data: Box<[u8]>
}

pub struct RawDef {
    type_: u32,
    colors: Box<[Color]>,
    names2sprites: HashMap<String, RawDefSprite>,
    blocks2names: HashMap<u32, Box<[String]>>
}

pub struct Animation<'a> {
    pub textures: Box<[Texture<'a>]>,
    pub blocks2indexes: HashMap<u32, Box<[usize]>>
}

impl LodIndex {
    pub fn open(path: &Path) -> Self {
        let mut f = File::open(path).unwrap();
        let mut parse_buffer: [u8; 16] = [0; 16];

        f.seek(SeekFrom::Start(8)).unwrap();

        f.read_exact(&mut parse_buffer).unwrap();
        let total_files = u32::from_le_bytes(parse_buffer[0..4].try_into().unwrap());

        f.seek(SeekFrom::Start(92)).unwrap();
        let mut file_registry: HashMap<String, LodFileInfo> = HashMap::with_capacity(total_files as usize);

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

            let file_info = LodFileInfo { offset, size, compressed: csize != 0 };
            file_registry.insert(filename, file_info);
        }
    
        LodIndex {handle: f, registry: file_registry}
    }

    pub fn read_file(&mut self, filename: &str) -> Box<[u8]> {
        let LodFileInfo { offset, size, compressed } = *self.registry.get(filename).unwrap();
        self.handle.seek(SeekFrom::Start(offset as u64)).unwrap();

        let reader: Box<dyn Read> = 
            if compressed {
                Box::new(ZlibDecoder::new(&mut self.handle))
            }
            else {
                Box::new(&self.handle)
            };
        reader
            .bytes()
            .take(size as usize)
            .map(Result::unwrap)
            .map(u8::from_le)
            .collect::<Box<[u8]>>()
    }
}


impl ResourceRegistry {
    pub fn init() -> Self {
        let pcx_archive = LodIndex::open(&[RESOURCES_ROOT, PCX_ARCHIVE].iter().collect::<PathBuf>());
        let def_archive = LodIndex::open(&[RESOURCES_ROOT, DEF_ARCHIVE].iter().collect::<PathBuf>());
        
        ResourceRegistry {
            pcx_archive,
            def_archive
        }
    }

    pub fn read_pcx_data(&mut self, filename: &str) -> RawPcx {
        let bytes = self.pcx_archive.read_file(filename);

        let (header, data) =  bytes.split_at(12); 
        let [size, width, height]: [u32; 3] = header
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .map(u32::from_ne_bytes)
            .collect::<Box<[u32]>>()
            .deref()
            .try_into()
            .unwrap();
        
        RawPcx {
            size,
            width,
            height,
            pixel_data: data.to_owned().into_boxed_slice()
        }
    }

    pub fn read_def_data(&mut self, filename: &str) -> RawDef {
        let bytes = self.def_archive.read_file(filename);
        let (header, payload) = bytes.split_at(16);

        let type_ = u32::from_le_bytes(header[0..4].try_into().unwrap());
        let n_blocks = u32::from_le_bytes(header[12..16].try_into().unwrap());

        let (palette_data, pixel_data) = payload.split_at(256 * 3);
        let colors = palette_data
            .chunks_exact(3)
            .map(|chunk| Color::RGB(chunk[0], chunk[1], chunk[2]))
            .collect::<Box<[Color]>>();
        
        let mut names2sprites: HashMap<String, RawDefSprite> = HashMap::new();
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
                        .map(|offset| RawDefSprite::from_bytes(bytes.deref(), offset));

            let block = names.clone().collect::<Box<[String]>>();
            blocks2names.insert(block_id, block);

            names2sprites.extend(Iterator::zip(names, sprites));
            
            rest_data
            });

        RawDef {type_, colors, names2sprites, blocks2names}
    }
}

impl RawPcx {
    fn construct_surface(&mut self) -> Surface {
        if self.size == self.width * self.height * 3 {
                Surface::from_data(&mut self.pixel_data, self.width, self.height, 3 * self.width, PixelFormatEnum::BGR24).unwrap()
        } 
        else {
            let (pixel_data, palette_data) = self.pixel_data.split_at_mut(self.size as usize);
            let colors = palette_data
                .chunks_exact(3)
                .map(|slice| Color::RGB(slice[0], slice[1], slice[2]))
                .collect::<Box<[Color]>>();
            let palette = Palette::with_colors(&colors).unwrap();

            let mut surface = Surface::from_data(pixel_data, self.width, self.height, 1 * self.width, PixelFormatEnum::Index8).unwrap();
            surface.set_palette(&palette).unwrap();
            surface
        }
    }
    pub fn to_texture<'a>(mut self, tc: &'a TextureCreator<WindowContext>) -> Texture<'a> {
        let surface = self.construct_surface();
        tc.create_texture_from_surface(surface).unwrap()
    }
}

impl RawDefSprite {
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
        RawDefSprite {
            size,
            full_width: fw,
            full_height: fh,
            width: w,
            height: h,
            left_margin: lm,
            top_margin: tm,
            pixel_data: pixel_data.into_boxed_slice()
        }
    }
}

impl RawDef {
    pub fn to_animation<'a>(self, tc: &'a TextureCreator<WindowContext>) -> Animation<'a> {
        let mut adjusted_colors = self.colors;
        adjusted_colors[0] = Color::RGBA(0, 0, 0, 0);
        adjusted_colors[1] = Color::RGBA(0, 0, 0, 32);
        adjusted_colors[4] = Color::RGBA(0, 0, 0, 128);
        adjusted_colors[5] = Color::RGBA(0, 255, 255, 255);
        adjusted_colors[6] = Color::RGBA(0, 0, 0, 128);
        adjusted_colors[7] = Color::RGBA(0, 0, 0, 64);

        let palette = Palette::with_colors(&adjusted_colors).unwrap();

        let (names2indexes, textures): (HashMap<String, usize>, Vec<Texture>) = self.names2sprites
            .into_iter()
            .enumerate()
            .map(|(index, (name, sprite))| {
                // Конвертируем байты в текстуру
                let RawDefSprite { width, height, mut pixel_data, .. } = sprite;
                let mut surface = Surface::from_data(&mut pixel_data, width, height, 1 * width, PixelFormatEnum::Index8).unwrap();
                surface.set_palette(&palette).unwrap();
                surface.set_color_key(true, adjusted_colors[0]).unwrap();
                let texture = tc.create_texture_from_surface(surface).unwrap();
                // Готовимся отказаться от строков индексов в пользу usize
                ((name, index), texture)
            })
            .unzip();
        
        let blocks2indexes: HashMap<u32, Box<[usize]>> = self.blocks2names
            .into_iter()
            .map(|(block_id, names)| {
                let indexes = names
                    .iter()
                    .map(|name| *names2indexes.get(name).unwrap())
                    .collect();
                (block_id, indexes)
            })
            .collect();
        
        Animation { textures: textures.into_boxed_slice(), blocks2indexes }
    }
}