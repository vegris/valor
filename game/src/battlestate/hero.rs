use gamedata::heroes::abilities::{Ability, HeroAbility, Level};
use gamedata::heroes::{Hero as GDHero, Stats};

pub struct Hero {
    pub hero: GDHero,
    pub stats: Stats,
    pub abilities: [Option<HeroAbility>; 7],
}

impl Hero {
    pub fn build(hero: GDHero) -> Self {
        const NONE: Option<HeroAbility> = None;
        let mut abilities: [Option<HeroAbility>; 7] = [NONE; 7];

        for (i, item) in hero.starting_abilities().into_iter().enumerate() {
            abilities[i] = item;
        }

        Self {
            hero,
            stats: hero.class().starting_stats(),
            abilities,
        }
    }

    pub fn get_ability_level(&self, ability: Ability) -> Option<Level> {
        self.abilities
            .into_iter()
            .find_map(|v| v.filter(|ha| ha.ability == ability).map(|ha| ha.level))
    }
}
