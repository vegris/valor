use egui::TextureId;
use gamedata::spells::Spell;

use crate::graphics::statics::{ButtonState, Buttons};

pub enum Texture {
    Button(Button),
    Spell(Spell),
}

impl TryFrom<TextureId> for Texture {
    type Error = &'static str;

    fn try_from(value: TextureId) -> Result<Self, Self::Error> {
        match value {
            TextureId::Managed(_) => Err("Managed textures can't be converted to Texture"),
            TextureId::User(texture_id) => {
                let texture = match texture_id {
                    0 => Self::Button(Button(Buttons::Surrender, ButtonState::Base)),
                    1 => Self::Button(Button(Buttons::Surrender, ButtonState::Pressed)),
                    2 => Self::Button(Button(Buttons::Surrender, ButtonState::Disabled)),
                    3 => Self::Button(Button(Buttons::Surrender, ButtonState::Hovered)),

                    10 => Self::Button(Button(Buttons::Retreat, ButtonState::Base)),
                    11 => Self::Button(Button(Buttons::Retreat, ButtonState::Pressed)),
                    12 => Self::Button(Button(Buttons::Retreat, ButtonState::Disabled)),
                    13 => Self::Button(Button(Buttons::Retreat, ButtonState::Hovered)),

                    20 => Self::Button(Button(Buttons::Settings, ButtonState::Base)),
                    21 => Self::Button(Button(Buttons::Settings, ButtonState::Pressed)),
                    22 => Self::Button(Button(Buttons::Settings, ButtonState::Disabled)),
                    23 => Self::Button(Button(Buttons::Settings, ButtonState::Hovered)),

                    30 => Self::Button(Button(Buttons::AutoBattle, ButtonState::Base)),
                    31 => Self::Button(Button(Buttons::AutoBattle, ButtonState::Pressed)),
                    32 => Self::Button(Button(Buttons::AutoBattle, ButtonState::Disabled)),
                    33 => Self::Button(Button(Buttons::AutoBattle, ButtonState::Hovered)),

                    40 => Self::Button(Button(Buttons::BookOfMagic, ButtonState::Base)),
                    41 => Self::Button(Button(Buttons::BookOfMagic, ButtonState::Pressed)),
                    42 => Self::Button(Button(Buttons::BookOfMagic, ButtonState::Disabled)),
                    43 => Self::Button(Button(Buttons::BookOfMagic, ButtonState::Hovered)),

                    50 => Self::Button(Button(Buttons::Wait, ButtonState::Base)),
                    51 => Self::Button(Button(Buttons::Wait, ButtonState::Pressed)),
                    52 => Self::Button(Button(Buttons::Wait, ButtonState::Disabled)),
                    53 => Self::Button(Button(Buttons::Wait, ButtonState::Hovered)),

                    60 => Self::Button(Button(Buttons::Defend, ButtonState::Base)),
                    61 => Self::Button(Button(Buttons::Defend, ButtonState::Pressed)),
                    62 => Self::Button(Button(Buttons::Defend, ButtonState::Disabled)),
                    63 => Self::Button(Button(Buttons::Defend, ButtonState::Hovered)),

                    100 => Self::Spell(Spell::Armageddon),

                    _ => return Err("TextureId is out of range"),
                };

                Ok(texture)
            }
        }
    }
}

pub struct Button(pub Buttons, pub ButtonState);

impl From<Button> for TextureId {
    fn from(val: Button) -> Self {
        let value = (val.0, val.1);

        let texture_id = match value {
            (Buttons::Surrender, ButtonState::Base) => 0,
            (Buttons::Surrender, ButtonState::Pressed) => 1,
            (Buttons::Surrender, ButtonState::Disabled) => 2,
            (Buttons::Surrender, ButtonState::Hovered) => 3,

            (Buttons::Retreat, ButtonState::Base) => 10,
            (Buttons::Retreat, ButtonState::Pressed) => 11,
            (Buttons::Retreat, ButtonState::Disabled) => 12,
            (Buttons::Retreat, ButtonState::Hovered) => 13,

            (Buttons::Settings, ButtonState::Base) => 20,
            (Buttons::Settings, ButtonState::Pressed) => 21,
            (Buttons::Settings, ButtonState::Disabled) => 22,
            (Buttons::Settings, ButtonState::Hovered) => 23,

            (Buttons::AutoBattle, ButtonState::Base) => 30,
            (Buttons::AutoBattle, ButtonState::Pressed) => 31,
            (Buttons::AutoBattle, ButtonState::Disabled) => 32,
            (Buttons::AutoBattle, ButtonState::Hovered) => 33,

            (Buttons::BookOfMagic, ButtonState::Base) => 40,
            (Buttons::BookOfMagic, ButtonState::Pressed) => 41,
            (Buttons::BookOfMagic, ButtonState::Disabled) => 42,
            (Buttons::BookOfMagic, ButtonState::Hovered) => 43,

            (Buttons::Wait, ButtonState::Base) => 50,
            (Buttons::Wait, ButtonState::Pressed) => 51,
            (Buttons::Wait, ButtonState::Disabled) => 52,
            (Buttons::Wait, ButtonState::Hovered) => 53,

            (Buttons::Defend, ButtonState::Base) => 60,
            (Buttons::Defend, ButtonState::Pressed) => 61,
            (Buttons::Defend, ButtonState::Disabled) => 62,
            (Buttons::Defend, ButtonState::Hovered) => 63,
        };

        TextureId::User(texture_id)
    }
}

pub fn convert_spell(spell: Spell) -> TextureId {
    let texture_id = match spell {
        Spell::Armageddon => 100,
        _ => todo!(),
    };
    TextureId::User(texture_id)
}
