use std::ops::Deref;

pub struct Sprite {
    pub full_size: u32,
    pub full_width: u32,
    pub full_height: u32,
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
    pub pixels: Box<[u8]>,
}

struct Header {
    full_size: u32,
    full_width: u32,
    full_height: u32,
    width: u32,
    height: u32,
    left_margin: u32,
    top_margin: u32,
    format: Format,
}

enum Format {
    Raw,
    Offsets,
    SegmentedOffsets,
    SegmentedOffsets32,
}

const HEADER_LENGTH: usize = 32;

impl Sprite {
    pub fn from_bytes(def_data: &[u8], offset: u32) -> Self {
        let data = &def_data[offset as usize..];
        let (header_data, image_data) = data.split_at(HEADER_LENGTH);

        let header = parse_header(header_data);
        let pixels = parse_pixel_data(&header, image_data);

        Self {
            full_size: header.full_size,
            full_width: header.full_width,
            full_height: header.full_height,
            width: header.width,
            height: header.height,
            left_margin: header.left_margin,
            top_margin: header.top_margin,
            pixels,
        }
    }
}

fn parse_header(header_data: &[u8]) -> Header {
    let [size, format, fw, fh, w, h, lm, tm]: [u32; 8] = header_data
        .chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(u32::from_le_bytes)
        .collect::<Box<[u32]>>()
        .deref()
        .try_into()
        .unwrap();

    let format = match format {
        0 => Format::Raw,
        1 => Format::Offsets,
        2 => Format::SegmentedOffsets,
        3 => Format::SegmentedOffsets32,
        _ => panic!("Unknown format"),
    };

    Header {
        full_size: size,
        full_width: fw,
        full_height: fh,
        width: w,
        height: h,
        left_margin: lm,
        top_margin: tm,
        format,
    }
}

fn parse_pixel_data(header: &Header, image_data: &[u8]) -> Box<[u8]> {
    let size = (header.width * header.height) as usize;

    let mut pixel_data: Vec<u8> = Vec::with_capacity(size);

    match header.format {
        Format::Raw => pixel_data.extend_from_slice(&image_data[..size]),
        Format::Offsets => parse_offsets(header, image_data, &mut pixel_data),
        Format::SegmentedOffsets => parse_segmented_offsets(header, image_data, &mut pixel_data),
        Format::SegmentedOffsets32 => {
            parse_segmented_offsets_32(header, image_data, &mut pixel_data)
        }
    }

    pixel_data.into_boxed_slice()
}

fn parse_offsets(header: &Header, image_data: &[u8], pixel_data: &mut Vec<u8>) {
    let line_offsets = image_data[..(4 * header.height) as usize]
        .chunks_exact(4)
        .map(|chunk| chunk.try_into().unwrap())
        .map(u32::from_ne_bytes);
    for line_offset in line_offsets {
        let mut row = &image_data[line_offset as usize..];
        let mut total_row_length = 0;
        while total_row_length < header.width {
            let (code, length) = (row[0], row[1] as u32 + 1); // idk why we need to +1 length
            match code {
                0xff => {
                    pixel_data.extend_from_slice(&row[2..length as usize + 2]);
                    row = &row[length as usize + 2..];
                }
                _ => {
                    for _ in 0..length {
                        pixel_data.push(code)
                    }
                    row = &row[2..];
                }
            }
            total_row_length += length;
        }
    }
}

fn parse_segmented_offsets(header: &Header, image_data: &[u8], pixel_data: &mut Vec<u8>) {
    let line_offsets = image_data[..(2 * header.height) as usize]
        .chunks_exact(2)
        .map(|chunk| chunk.try_into().unwrap())
        .map(u16::from_ne_bytes);
    for line_offset in line_offsets {
        let mut row = &image_data[line_offset as usize..];
        let mut total_row_length = 0;
        while total_row_length < header.width {
            let segment = row[0];
            let code = segment >> 5;
            let length = (segment & 0x1f) + 1;
            match code {
                7 => {
                    pixel_data.extend_from_slice(&row[1..length as usize + 1]);
                    row = &row[length as usize + 1..];
                }
                _ => {
                    for _ in 0..length {
                        pixel_data.push(code)
                    }
                    row = &row[1..];
                }
            }
            total_row_length += length as u32;
        }
    }
}

fn parse_segmented_offsets_32(header: &Header, image_data: &[u8], pixel_data: &mut Vec<u8>) {
    let size = (header.width * header.height) as usize;
    // each row is split into 32 byte long blocks which are individually encoded
    // two bytes store the offset for each block per line
    let line_offsets = image_data
        .chunks_exact(2)
        .take(size / 32)
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
                }
                _ => {
                    for _ in 0..length {
                        pixel_data.push(code)
                    }
                    row = &row[1..];
                }
            }
            total_block_length += length as u32;
        }
    }
}
