use crate::Creature;

pub enum Ability {
    DoubleShot,
    Hatred { to: Box<[Creature]> },
}
