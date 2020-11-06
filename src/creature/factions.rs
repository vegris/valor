use super::Creature;

#[derive(Clone, Copy, PartialEq)]
pub enum Faction {
    Castle,
    Rampart,
    Tower,
    Inferno,
    Necropolis,
    Dungeon,
    Stronghold,
    Fortress,
    Conflux,
    Neutral,
    WarMachines,
}

impl Creature {
    pub fn faction(&self) -> Faction {
        [
            (Self::Pikeman,    Self::Archangel,       Faction::Castle),
            (Self::Centaur,    Self::GoldDragon,      Faction::Rampart),
            (Self::Gremlin,    Self::Titan,           Faction::Tower),
            (Self::Imp,        Self::ArchDevil,       Faction::Inferno),
            (Self::Skeleton,   Self::GhostDragon,     Faction::Necropolis),
            (Self::Troglodyte, Self::BlackDragon,     Faction::Dungeon),
            (Self::Goblin,     Self::AncientBehemoth, Faction::Stronghold),
            (Self::Gnoll,      Self::ChaosHydra,      Faction::Fortress),
            (Self::Pixie,      Self::Phoenix,         Faction::Conflux),
            (Self::Peasant,    Self::AzureDragon,     Faction::Neutral),
            (Self::Ballista,   Self::AmmoCart,        Faction::WarMachines),
        ]
        .iter()
        .find(|&&(first, last, _town)| (first..=last).contains(self))
        .map(|&(_first, _last, town)| town)
        .expect("Creature without faction - this should not happen!")
    }
}
