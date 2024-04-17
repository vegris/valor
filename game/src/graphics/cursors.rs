use sdl2::mouse::Cursor as SDLCursor;

use strum::IntoEnumIterator;

use gamedata::cursors::Cursor;

use crate::resources::spritesheets::{ContainerType, SpriteGroupT};
use crate::resources::ResourceRegistry;
use logic::grid::AttackDirection;

pub fn from_attack_direction(attack_direction: AttackDirection) -> Cursor {
    match attack_direction {
        AttackDirection::Left => Cursor::AttackLeft,
        AttackDirection::TopLeft => Cursor::AttackUpLeft,
        AttackDirection::Top => Cursor::AttackUp,
        AttackDirection::TopRight => Cursor::AttackUpRight,
        AttackDirection::Right => Cursor::AttackRight,
        AttackDirection::BottomRight => Cursor::AttackDownRight,
        AttackDirection::Bottom => Cursor::AttackDown,
        AttackDirection::BottomLeft => Cursor::AttackDownLeft,
    }
}

impl ContainerType for Cursor {
    const CONTAINER_TYPE: u32 = Cursor::CONTAINER_TYPE;
}

impl SpriteGroupT for Cursor {}

pub struct Cursors(Box<[SDLCursor]>);

impl Cursors {
    pub fn load(rr: &mut ResourceRegistry) -> Self {
        let sprite_group = rr.load_sprite_group::<Cursor>(Cursor::CONTAINTER_FILENAME);

        let cursors = sprite_group
            .into_sprites()
            .into_vec() // Boxed slice problems...
            .into_iter()
            .zip(Cursor::iter())
            .map(|(sprite, cursor)| {
                let (off_x, off_y) = cursor.pointer_offset();
                SDLCursor::from_surface(sprite.surface, off_x, off_y).unwrap()
            })
            .collect();

        Self(cursors)
    }

    pub fn get(&self, cursor: Cursor) -> &SDLCursor {
        &self.0[cursor as usize]
    }
}
