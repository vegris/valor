use std::error::Error;

extern crate sdl2;
use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;

use creature::Creature;

use crate::creature_stack::{CreatureStack, CreatureTurnState as CTS};
use crate::gridpos::GridPos;
use crate::Battlefield;
use crate::registry::ResourceRegistry;
use crate::graphics::cursors::Cursors;

mod army;
mod turns;
mod input;
mod draw;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Side {
    Attacker,
    Defender
}

impl Side {
    pub fn other(self) -> Self {
        match self {
            Self::Attacker => Self::Defender,
            Self::Defender => Self::Attacker
        }
    }
}


pub struct BattleState<'a> {
    // Логика

    pub sides: [Vec<CreatureStack>; 2],
    pub phase_iter: turns::PhaseIterator,
    pub current_phase: CTS,
    pub last_turn_side: Side,
    pub current_side: Side,
    pub current_stack: usize,

    // Графика

    // Постоянно используемые текстуры,
    // которые нет смысла прокачивать сквозь кэш
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,

    cursors: Cursors,

    current_hover: Option<GridPos>,
}


impl<'a> BattleState<'a> {
    pub fn new(
        attacker_units: [Option<(Creature, u32)>; 7],
        defender_units: [Option<(Creature, u32)>; 7],
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
        battlefield: Battlefield
    ) -> Result<Self, Box<dyn Error>> {
        let attacker_army = army::form_units(&attacker_units, Side::Attacker);
        let defender_army = army::form_units(&defender_units, Side::Defender);

        let mut state = Self {
            sides: [attacker_army, defender_army],
            phase_iter: turns::new_phase_iter(),
            current_phase: CTS::HasTurn,
            last_turn_side: Side::Defender,
            current_side: Side::Attacker,
            current_stack: 0,

            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency("CCellGrd.pcx")?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency("CCellShd.pcx")?.as_texture(&tc)?,

            cursors: Cursors::load(rr),

            current_hover: None,
        };

        state.update_current_stack();
        Ok(state)
    }

    pub fn battle_army(&self, side: Side) -> &Vec<CreatureStack> {
        &self.sides[side as usize]
    }
    pub fn battle_army_mut(&mut self, side: Side) -> &mut Vec<CreatureStack> {
        &mut self.sides[side as usize]
    }

    pub fn get_stack(&self, side: Side, index: u8) -> Option<&CreatureStack> {
        self.sides[side as usize].get(index as usize)
    }
    pub fn get_stack_mut(&mut self, side: Side, index: u8) -> Option<&mut CreatureStack> {
        self.sides[side as usize].get_mut(index as usize)
    }

    pub fn current_stack_id(&self) -> (Side, u8) {
        (self.current_side, self.current_stack as u8)
    }
    pub fn get_current_stack(&self) -> &CreatureStack {
        &self.battle_army(self.current_side)[self.current_stack]
    }
    pub fn get_current_stack_mut(&mut self) -> &mut CreatureStack {
        let stack_id = self.current_stack;
        &mut self.battle_army_mut(self.current_side)[stack_id]
    }


    fn units(&self) -> Vec<(Side, &CreatureStack)> {
        self.sides
            .iter()
            .zip(vec![Side::Attacker, Side::Defender].into_iter())
            .map(|(units, side)| units.iter().map(move |unit| (side, unit)))
            .flatten()
            .collect()
    }

    fn find_selected_unit(&self, pos: GridPos) -> Option<(Side, &CreatureStack)> {
        self.units()
            .into_iter()
            .find(|(side, unit)| unit.get_occupied_cells(*side).contains(&pos))
    }
}
