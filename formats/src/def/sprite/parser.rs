use super::header::Header;
use super::format::Format;

pub fn parse_pixel_data(header: Header, image_data: &[u8]) -> Box<[u8]> {
    let Header { height, width, format, .. } = header;

    let size = (width * height) as usize;

    let mut pixel_data: Vec<u8> = Vec::with_capacity(size);

    match format {
        Format::Raw =>
            pixel_data.extend_from_slice(&image_data[..size]),
        Format::Offsets =>
            parse_offsets(header, image_data, &mut pixel_data),
        Format::SegmentedOffsets =>
            parse_segmented_offsets(header, image_data, &mut pixel_data),
        Format::SegmentedOffsets32 =>
            parse_segmented_offsets_32(header, image_data, &mut pixel_data)
    }

    pixel_data.into_boxed_slice()
}

fn parse_offsets(header: Header, image_data: &[u8], pixel_data: &mut Vec<u8>) {
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
                        for _ in 0..length { pixel_data.push(code) }; 
                        row = &row[2..];
                }
            }
            total_row_length += length;
        }
    }
}

fn parse_segmented_offsets(header: Header, image_data: &[u8], pixel_data: &mut Vec<u8>) {
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
                },
                _ => {
                    for _ in 0..length { pixel_data.push(code) };
                    row = &row[1..];
                }
            }
            total_row_length += length as u32;
        }
    }

}

fn parse_segmented_offsets_32(header: Header, image_data: &[u8], pixel_data: &mut Vec<u8>) {
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
