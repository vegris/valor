use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;
use std::convert::TryInto;

extern crate flate2;
use flate2::read::ZlibDecoder;


struct LodFileInfo {
    offset: u32,
    size: u32,
    compressed: bool
}

pub struct LodIndex {
    handle: File,
    registry: HashMap<String, LodFileInfo>
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
