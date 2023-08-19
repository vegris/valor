use std::fs::File;

use serde::Deserialize;

use gamedata::battlefields::Battlefield;
use gamedata::creatures::Creature;
use gamedata::heroes::Hero;

#[derive(Clone, Copy, Deserialize)]
pub struct Army {
    pub hero: Option<Hero>,
    pub stacks: [Option<(Creature, i32)>; 7],
}

#[derive(Deserialize)]
pub struct Config {
    pub battlefield: Battlefield,
    pub music: bool,
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
