use super::Event;
use crate::command::Cast;
use crate::event::Cast as EventCast;
use crate::gamestate::GameState;

pub fn is_applicable(_cast: Cast, _state: &GameState) -> bool {
    true
}
pub fn apply(cast: Cast, _state: &mut GameState) -> Vec<Event> {
    vec![Event::Cast(EventCast {
        spell: cast.spell,
        target: cast.target,
    })]
}
