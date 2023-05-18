use std::ops::Deref;

use super::format::Format;

#[derive(Clone, Copy)]
pub struct Header {
    pub full_size: u32,
    pub format: Format,
    pub full_width: u32,
    pub full_height: u32,
    pub width: u32,
    pub height: u32,
    pub left_margin: u32,
    pub top_margin: u32,
}

impl Header {
    pub const LENGTH: usize = 32;

    pub fn parse(header_data: &[u8]) -> Self {
        let [size, format, fw, fh, w, h, lm, tm]: [u32; 8] = header_data
            .chunks_exact(4)
            .map(TryInto::try_into)
            .map(Result::unwrap)
            .map(u32::from_le_bytes)
            .collect::<Box<[u32]>>()
            .deref()
            .try_into()
            .unwrap();

        Self {
            full_size: size,
            format: format.try_into().unwrap(),
            full_width: fw,
            full_height: fh,
            width: w,
            height: h,
            left_margin: lm,
            top_margin: tm,
        }
    }
}
