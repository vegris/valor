use super::Creature;

#[derive(Clone, Copy)]
pub enum CreatureAbility {
    IgnoreDefence { percent: u8 },
    CavalierBonus,
    Hatred { creature: Creature, value: f32 },
    NoMeleePenalty,
    NoObstaclePenalty
}

impl PartialEq for CreatureAbility {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

type C  = Creature;
type CA = CreatureAbility;

impl Creature {
    pub fn abilities(&self) -> Box<[CreatureAbility]> {
        match self {
            C::Behemoth => vec![CA::IgnoreDefence { percent: 40 }],
            C::AncientBehemoth => vec![CA::IgnoreDefence { percent: 80 }],
            C::Cavalier => vec![CA::CavalierBonus],
            C::Champion => vec![CA::CavalierBonus],
            C::Angel => vec![CA::Hatred { creature: C::Devil, value: 1.5 }],
            _ => vec![]
        }.into_boxed_slice()
    }
}