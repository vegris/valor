use std::error::Error;

extern crate sdl2;
use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;

use creature::Creature;

use crate::creature_stack::{CreatureStack, CreatureTurnState as CTS};
use crate::gridpos::GridPos;
use crate::pathfinding::NavigationArray;
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

#[derive(Clone, Copy, Debug)]
pub struct CreatureStackHandle {
    pub side: Side,
    pub index: u8
}

pub struct BattleState<'a> {
    // Логика

    pub sides: [Vec<CreatureStack>; 2],
    pub phase_iter: turns::PhaseIterator,
    pub current_phase: CTS,
    pub last_turn_side: Side,
    pub current_stack: CreatureStackHandle,

    // Поиск пути
    pub navigation_array: NavigationArray,
    pub reachable_cells: Vec<GridPos>,

    // Графика

    // Постоянно используемые текстуры,
    // которые нет смысла прокачивать сквозь кэш
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,
    stack_count_bg: Texture<'a>,

    cursors: Cursors
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
            current_stack: CreatureStackHandle { side: Side::Attacker, index: 0 },
            navigation_array: NavigationArray::empty(),
            reachable_cells: vec![],

            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency("CCellGrd.pcx")?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency("CCellShd.pcx")?.as_texture(&tc)?,
            stack_count_bg: rr.load_pcx("CmNumWin.pcx")?.as_texture(&tc)?,

            cursors: Cursors::load(rr)
        };

        state.update_current_stack();
        Ok(state)
    }

    pub fn get_stack(&self, handle: CreatureStackHandle) -> &CreatureStack {
        &self.sides[handle.side as usize][handle.index as usize]
    }
    pub fn get_stack_mut(&mut self, handle: CreatureStackHandle) -> &mut CreatureStack {
        &mut self.sides[handle.side as usize][handle.index as usize]
    }

    pub fn get_current_stack(&self) -> &CreatureStack {
        self.get_stack(self.current_stack)
    }

    pub fn get_current_stack_mut(&mut self) -> &mut CreatureStack {
        self.get_stack_mut(self.current_stack)
    }

    pub fn get_current_side(&self) -> Side {
        self.current_stack.side
    }

    pub fn units(&self) -> Vec<CreatureStackHandle> {
        let total_units = self.sides.iter().map(|side| side.len()).sum();
        let mut units = Vec::with_capacity(total_units);
        for side in vec![Side::Attacker, Side::Defender].into_iter() {
            for index in 0..self.sides[side as usize].len() {
                let handle = CreatureStackHandle { side, index: index as u8};
                units.push(handle);
            }
        }
        units
    }

    pub fn find_unit_for_cell(&self, cell: GridPos) -> Option<CreatureStackHandle> {
        self.units()
            .into_iter()
            .find(|&handle| {
                let stack = self.get_stack(handle);
                let occupied_cells = stack.get_occupied_cells(handle.side);
                occupied_cells.contains(&cell)
            })
    }
}
