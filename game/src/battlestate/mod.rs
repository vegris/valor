use std::error::Error;
use std::collections::HashMap;

extern crate sdl2;
use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;

use gridpos::GridPos;

use crate::creature_stack::{CreatureStack, CreatureTurnState as CTS};
use crate::pathfinding::NavigationArray;
use crate::registry::ResourceRegistry;
use crate::graphics::cursors::Cursors;
use crate::config::Config;

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

#[derive(Debug)]
pub enum Winner {
    Side(Side),
    Tie
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CreatureStackHandle(u32);

pub struct BattleState<'a> {
    // Логика

    pub stacks: HashMap<CreatureStackHandle, CreatureStack>,
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
        config: Config,
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>
    ) -> Result<Self, Box<dyn Error>> {

        let attacker_army = army::form_units(&config.armies[0], Side::Attacker);
        let defender_army = army::form_units(&config.armies[1], Side::Defender);

        let stacks = [attacker_army, defender_army]
            .concat()
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let handle = CreatureStackHandle(i as u32);
                (handle, v)
            })
            .collect();

        let mut state = Self {
            stacks,
            phase_iter: turns::new_phase_iter(),
            current_phase: CTS::HasTurn,
            last_turn_side: Side::Defender,
            current_stack: CreatureStackHandle(0),
            navigation_array: NavigationArray::empty(),
            reachable_cells: vec![],

            battlefield: rr.load_pcx(config.battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency("CCellGrd.pcx")?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency("CCellShd.pcx")?.as_texture(&tc)?,
            stack_count_bg: rr.load_pcx("CmNumWin.pcx")?.as_texture(&tc)?,

            cursors: Cursors::load(rr)
        };

        state.update_current_stack();
        Ok(state)
    }

    pub fn get_stack(&self, handle: CreatureStackHandle) -> &CreatureStack {
        &self.stacks[&handle]
    }
    pub fn get_stack_mut(&mut self, handle: CreatureStackHandle) -> &mut CreatureStack {
        self.stacks.get_mut(&handle).unwrap()
    }

    pub fn get_current_stack(&self) -> &CreatureStack {
        self.get_stack(self.current_stack)
    }

    pub fn get_current_stack_mut(&mut self) -> &mut CreatureStack {
        self.get_stack_mut(self.current_stack)
    }

    pub fn units(&self) -> Vec<CreatureStackHandle> {
        self.stacks.keys().copied().collect()
    }

    pub fn find_unit_for_cell(&self, cell: GridPos) -> Option<CreatureStackHandle> {
        self.units()
            .into_iter()
            .filter(|&handle| self.get_stack(handle).is_alive())
            .find(|&handle| {
                self.get_stack(handle)
                    .get_occupied_cells()
                    .contains(&cell)
            })
    }


    pub fn find_winner(&self) -> Option<Winner> {
        let alive_sides = 
            [Side::Attacker, Side::Defender]
                .into_iter()
                .filter(|&side| {
                    self.stacks
                        .values()
                        .filter(|stack| stack.side == side)
                        .any(|stack| stack.is_alive())
                })
                .collect::<Vec<Side>>();
        
        match alive_sides.len() {
            0 => Some(Winner::Tie),
            1 => Some(Winner::Side(alive_sides[0])),
            2 => None,
            _ => unreachable!()
        }
    }
}
