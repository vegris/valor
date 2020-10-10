use std::collections::VecDeque;

use super::creature::Creature;

struct CreatureStack {
    creature: Creature,
    count: u32,
    side: Side
}

struct CreatureStackIndex(usize);

const CREATURE_LIMIT_FOR_EACH_SIDE: usize = 20;
struct Army {
    hero: Hero,
    creatures: [Option<(u32, Creature)>; CREATURE_LIMIT_FOR_EACH_SIDE],
}

enum Side {
    Attacker = 0,
    Defender = 1
}

struct BattleState {
    armies: [Army; 2],
    actions_queue: VecDeque<BattleAction>
}

struct Hero {
    attack: u8,
    defence: u8,
    spell_power: u8,
    knowledge: u8
}

struct BattleAction {
    commander: Side,
    action_type: BattleActionType
}

enum BattleActionType {
    Attack {
        target_index: CreatureStackIndex
    }
}
