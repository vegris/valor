use super::skills::{SkillLevel, Spell};

pub struct Hero {
    pub level: u8,

    pub attack: u8,
    pub defence: u8,
    pub spell_power: u8,
    pub knowledge: u8,
    
    pub specialty: HeroSpecialty,
    pub skills: Vec<(HeroAbility, SkillLevel)>,
    pub artifacts: Vec<Artifact>
}

#[derive(PartialEq, Clone, Copy)]
pub enum HeroAbility {
    Offense,
    Archery,
    Armorer,
    Navigation,
    Wisdom
}

#[derive(PartialEq, Clone, Copy)]
pub enum Artifact {
    BowOfElvenCherrywood,
    BowstringOfTheUnicorn,
    AngelFeatherArrows,
    BowOfTheSharpshooter
}

#[derive(PartialEq, Clone, Copy)]
pub enum HeroSpecialty {
    HeroAbility(HeroAbility),
    Spell(Spell)
}

impl Hero {
    pub fn get_skill(&self, ability: HeroAbility) -> Option<SkillLevel> {
        self.skills.iter().find(|(a, l)| *a == ability).map(|(a, l)| *l)
    }
    pub fn has_artifact(&self, artifact: Artifact) -> bool {
        self.artifacts.iter().find(|&&a| a == artifact).is_some()
    }
}
