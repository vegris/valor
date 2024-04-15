use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::ops::Deref;
use std::path::Path;

extern crate flate2;
use flate2::read::ZlibDecoder;

const FILE_INFO_SIZE: usize = 32;

struct LodFileInfo {
    offset: u32,
    size: u32,
    compressed: bool,
}

pub struct LodIndex {
    handle: File,
    registry: HashMap<String, LodFileInfo>,
}

impl LodIndex {
    pub fn open<T: AsRef<Path>>(path: T) -> Self {
        let mut f = File::open(path).unwrap();
        let mut parse_buffer: [u8; 16] = [0; 16];

        f.seek(SeekFrom::Start(8)).unwrap();

        f.read_exact(&mut parse_buffer).unwrap();
        let total_files = u32::from_le_bytes(parse_buffer[0..4].try_into().unwrap());

        f.seek(SeekFrom::Start(92)).unwrap();

        let mut parse_buffer = vec![0; FILE_INFO_SIZE * total_files as usize];
        f.read_exact(&mut parse_buffer).unwrap();

        let registry = parse_buffer
            .chunks_exact(FILE_INFO_SIZE)
            .map(TryInto::try_into)
            .map(Result::unwrap)
            .map(parse_file_info)
            .collect();

        LodIndex {
            handle: f,
            registry,
        }
    }

    pub fn read_file(&mut self, filename: &str) -> Box<[u8]> {
        let LodFileInfo {
            offset,
            size,
            compressed,
        } = *self.registry.get(filename).unwrap();
        self.handle.seek(SeekFrom::Start(offset as u64)).unwrap();

        let reader: Box<dyn Read> = if compressed {
            Box::new(ZlibDecoder::new(&mut self.handle))
        } else {
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

fn parse_file_info(data: [u8; 32]) -> (String, LodFileInfo) {
    let str_bytes = data.split(|chr| *chr == 0).next().unwrap();
    let filename = String::from_utf8(str_bytes.to_vec()).unwrap();

    // Reading sizes and offset
    let [offset, size, _, compressed_size]: [u32; 4] = data[16..]
        .chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(u32::from_le_bytes)
        .collect::<Box<[u32]>>()
        .deref()
        .try_into()
        .unwrap();

    let compressed = compressed_size != 0;

    (
        filename,
        LodFileInfo {
            offset,
            size,
            compressed,
        },
    )
}
