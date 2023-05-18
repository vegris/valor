#[derive(Clone, Copy)]
pub enum Format {
    Raw,
    Offsets,
    SegmentedOffsets,
    SegmentedOffsets32,
}

impl TryFrom<u32> for Format {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Raw),
            1 => Ok(Self::Offsets),
            2 => Ok(Self::SegmentedOffsets),
            3 => Ok(Self::SegmentedOffsets32),
            _ => Err("Unknown format"),
        }
    }
}
