use std::fs::File;

extern crate serde;
use serde::Deserialize;

extern crate ron;

use gamedata::{heroes::Hero, Battlefield, Creature};

#[derive(Clone, Copy, Deserialize)]
pub struct Army {
    pub hero: Option<Hero>,
    pub stacks: [Option<(Creature, i32)>; 7],
}

#[derive(Deserialize)]
pub struct Config {
    pub battlefield: Battlefield,
    pub armies: [Army; 2],
}

impl Config {
    const CONFIG_FILE: &'static str = "config.ron";

    pub fn load() -> Result<Config, ron::Error> {
        let reader = File::open(Self::CONFIG_FILE)?;
        let config = ron::de::from_reader(reader)?;

        Ok(config)
    }
}
