#[derive(Clone, Copy)]
pub enum Creature {
    Champion,
    Peasant
}

impl Creature {
    pub fn filename(self) -> &'static str {
        FILENAMES[self as usize]
    }
    pub const fn count() -> usize {
        2
    }
}

const FILENAMES: [&str; Creature::count()] = [
    "CCHAMP.def",
    "Cpeas.def"
];

#[derive(Clone, Copy)]
#[allow(unused)]
pub enum AnimationType {
    Moving = 0,
    MouseOver = 1,
    Standing = 2,
    GettingHit = 3,
    Defend = 4,
    Death = 5,
    UnusedDeath = 6,
    TurnLeft = 7,
    TurnRight = 8,
    // These are the same
    // TurnLeft = 9,
    // TurnRight = 10
    AttackUp = 11,
    AttackStraight = 12,
    AttackDown = 13,
    ShootUp = 14,
    ShootStraight = 15,
    ShootDown = 16,
    TwoHexAttackUp = 17,
    TwoHexAttackStraight = 18,
    TwoHexAttackDown = 19,
    StartMoving = 20,
    StopMoving = 21
}