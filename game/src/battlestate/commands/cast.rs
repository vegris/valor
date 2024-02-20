use crate::{battlestate::BattleState, command::Cast};

use super::Event;
use crate::event::Cast as EventCast;

pub fn is_applicable(_cast: Cast, _state: &BattleState) -> bool {
    true
}
pub fn apply(cast: Cast, _state: &mut BattleState) -> Vec<Event> {
    vec![Event::Cast(EventCast {
        spell: cast.spell,
        target: cast.target,
    })]
}
