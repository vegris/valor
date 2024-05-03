use std::fs::File;

use gamedata::battlefields::Battlefield;
use logic::gamestate::Army;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub battlefield: Battlefield,
    pub music: bool,
    pub volume: i32,
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
