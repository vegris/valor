use std::fs::File;

extern crate serde;
use serde::Deserialize;

extern crate ron;

use gamedata::{Battlefield, Creature};

type Army = [Option<(Creature, u32)>; 7];

#[derive(Deserialize)]
pub struct Config {
    pub battlefield: Battlefield,
    pub armies: [Army; 2]
}

impl Config {
    const CONFIG_FILE: &'static str = "config.ron";

    const DEFAULT_CONFIG: Config = Config {
        battlefield: Battlefield::GRMT,
        armies: [
            [
                Some((Creature::Archer, 55)),
                Some((Creature::Archangel, 8)),
                None,
                Some((Creature::RoyalGriffin, 30)),
                None,
                None,
                None
            ],
            [
                None,
                None,
                Some((Creature::Devil, 10)),
                Some((Creature::Angel, 20)),
                Some((Creature::GoldDragon, 1)),
                Some((Creature::HornedDemon, 25)),
                None
            ]
        ]
    };

    pub fn load_config() -> Result<Config, ron::Error> {
        if let Ok(f) = File::open(Self::CONFIG_FILE) {
            ron::de::from_reader(f)
        } else {
            Ok(Self::DEFAULT_CONFIG)
        }
    }
}
