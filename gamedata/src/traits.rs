use common::EnumIndex;
use strum::{EnumCount, IntoEnumIterator};

pub trait SpriteGroupT: ContainerType + EnumCount + EnumIndex {}
pub trait AnimationGroupT: Copy + ContainerType + EnumCount + EnumIndex + IntoEnumIterator {
    fn container_index(self) -> u32;
}

pub trait ContainerType {
    const CONTAINER_TYPE: u32;
}
