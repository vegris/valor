pub mod error;
pub mod map;

pub trait EnumIndex {
    fn into_index(self) -> usize;
}
