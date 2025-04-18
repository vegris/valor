use gamedata::heroes::{Ability, AbilityLevel, Hero as GDHero, LearnedAbility, Stats};

pub struct Hero {
    pub hero: GDHero,
    pub stats: Stats,
    pub abilities: [Option<LearnedAbility>; 7],
}

impl Hero {
    pub fn build(hero: GDHero) -> Self {
        const NONE: Option<LearnedAbility> = None;
        let mut abilities: [Option<LearnedAbility>; 7] = [NONE; 7];

        for (i, item) in hero.starting_abilities().into_iter().enumerate() {
            abilities[i] = item;
        }

        Self {
            hero,
            stats: hero.class().starting_stats(),
            abilities,
        }
    }

    pub fn get_ability_level(&self, ability: Ability) -> Option<AbilityLevel> {
        self.abilities
            .into_iter()
            .find_map(|v| v.filter(|ha| ha.ability == ability).map(|ha| ha.level))
    }
}
