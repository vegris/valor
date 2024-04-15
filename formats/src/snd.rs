use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, SeekFrom},
    ops::Deref,
    path::Path,
};

struct SndFileInfo {
    offset: u32,
    size: u32,
}

pub struct SndIndex {
    handle: File,
    registry: HashMap<String, SndFileInfo>,
}

const FILE_INFO_SIZE: usize = 48;

impl SndIndex {
    pub fn open<T: AsRef<Path>>(path: T) -> Self {
        let mut f = File::open(path).unwrap();
        let mut parse_buffer: [u8; 4] = [0; 4];

        f.read_exact(&mut parse_buffer).unwrap();
        let total_files = u32::from_le_bytes(parse_buffer);

        let mut parse_buffer = vec![0; FILE_INFO_SIZE * total_files as usize];
        f.read_exact(&mut parse_buffer).unwrap();

        let registry = parse_buffer
            .chunks_exact(FILE_INFO_SIZE)
            .map(TryInto::try_into)
            .map(Result::unwrap)
            .map(parse_file_info)
            .collect();

        Self {
            handle: f,
            registry,
        }
    }

    pub fn read_file(&mut self, filename: &str) -> Box<[u8]> {
        let SndFileInfo { offset, size } = *self.registry.get(filename).unwrap();
        self.handle.seek(SeekFrom::Start(offset as u64)).unwrap();

        let mut buffer = vec![0; size as usize];
        self.handle.read_exact(&mut buffer).unwrap();

        buffer.into_iter().map(u8::from_le).collect()
    }
}

fn parse_file_info(data: [u8; FILE_INFO_SIZE]) -> (String, SndFileInfo) {
    let str_bytes = data.split(|chr| *chr == 0).next().unwrap();
    let filename = String::from_utf8(str_bytes.to_vec()).unwrap();

    // Reading sizes and offset
    let [offset, size]: [u32; 2] = data[40..48]
        .chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(u32::from_le_bytes)
        .collect::<Box<[u32]>>()
        .deref()
        .try_into()
        .unwrap();

    (filename, SndFileInfo { offset, size })
}
