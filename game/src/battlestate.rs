use std::time::Duration;
use std::error::Error;

extern crate sdl2;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::{Rect, Point};
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate itertools;
use itertools::iproduct;

use creature::Creature;
use super::creature_stack::{CreatureStack, CreatureTurnState as CTS};
use super::gridpos::{GridPos, HexagonPart};
use crate::Battlefield;
use crate::resources::ResourceRegistry;
use crate::graphics::cursors::{Cursors, Cursor};

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

type PhaseIterator = std::vec::IntoIter<CTS>;

pub struct BattleState<'a> {
    // Логика

    pub sides: [Vec<CreatureStack>; 2],
    pub phase_iter: PhaseIterator,
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

fn initial_placement(units_count: u8) -> Vec<u16> {
    match units_count {
        1 => vec![6],
        2 => vec![3, 9],
        3 => vec![3, 6, 9],
        4 => vec![1, 5, 7, 11],
        5 => vec![1, 3, 6, 9, 11],
        6 => vec![1, 3, 5, 7, 9, 11],
        7 => vec![1, 3, 5, 6, 7, 9, 11],
        _ => unreachable!()
    }
}

fn form_units(starting_army: &[Option<(Creature, u32)>; 7], side: Side) -> Vec<CreatureStack> {
    let units_count = starting_army.iter().filter(|c| c.is_some()).count();
    let formation = initial_placement(units_count as u8);
    let starting_x = *match side {
        Side::Attacker => GridPos::X_RANGE.start(),
        Side::Defender => GridPos::X_RANGE.end()
    };
    starting_army
        .into_iter()
        .filter_map(|c| *c)
        .zip(formation.into_iter())
        .map(|((creature, count), y_pos)| {
            CreatureStack::new(creature, count, GridPos::new(starting_x, y_pos), side)
        })
        .collect()
}

fn new_phase_iter() -> PhaseIterator {
    vec![CTS::HasTurn, CTS::Waited].into_iter()
}

impl<'a> BattleState<'a> {
    pub fn new(
        attacker_units: [Option<(Creature, u32)>; 7],
        defender_units: [Option<(Creature, u32)>; 7],
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
        battlefield: Battlefield
    ) -> Result<Self, Box<dyn Error>> {
        let attacker_army = form_units(&attacker_units, Side::Attacker);
        let defender_army = form_units(&defender_units, Side::Defender);

        let mut state = Self {
            sides: [attacker_army, defender_army],
            phase_iter: new_phase_iter(),
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

    pub fn update_current_stack(&mut self) {
        if let Some((side, index)) = self.find_current_creature() {
            self.current_side = side;
            self.current_stack = index;
            let mut stack = self.get_current_stack_mut();
            stack.defending = false;
            println!("Current stack is {}, {:?}", stack, side);
        } else {
            self.advance_phase();
            self.update_current_stack();
        }
    }

    pub fn advance_phase(&mut self) {
        if let Some(phase) = self.phase_iter.next() {
            self.current_phase = phase;
            println!("New turn phase: {:?}!", self.current_phase);
        } else {
            self.new_turn();
            self.advance_phase();
        }
    }

    pub fn new_turn(&mut self) {
        self.phase_iter = new_phase_iter();
        self.sides
            .iter_mut()
            .flatten()
            .for_each(|creature| creature.turn_state = CTS::HasTurn);
        self.last_turn_side = self.last_turn_side.other();
        println!("New turn!");
    }

    fn find_current_creature(&self) -> Option<(Side, usize)> {
        // Преимущество при равенстве скоростей у того кто ходил вторым на прошлом ходу
        match self.last_turn_side {
            Side::Attacker => vec![Side::Defender, Side::Attacker],
            Side::Defender => vec![Side::Attacker, Side::Defender]
        }
        .into_iter()
        .flat_map(|side| Iterator::zip(
            std::iter::repeat(side),
            self.battle_army(side).iter().enumerate()
        ))
        .map(|(side, (index, stack))| (side, index, stack)) // чтоб не утонуть в скобках
        .filter(|(_side, _index, stack)| stack.turn_state == self.current_phase)
        .fold(None, |acc, current| {
            // Без max_first тяжко
            fn key((_, _, stack): (Side, usize, &CreatureStack)) -> u8 {
                stack.speed()
            };
            match acc {
                None => Some(current),
                Some(acc) if key(current) > key(acc) => Some(current),
                _ => acc
            }
        })
        .map(|(side, index, _stack)| (side, index))
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

    // Графика

    pub fn process_input(&mut self, event_pump: &mut EventPump) {
        // Опрашиваем устройства
        event_pump.pump_events();

        // Получаем позицию мыши
        let mouse_state = event_pump.mouse_state();
        let point = (mouse_state.x(), mouse_state.y());

        // Текущая клетка под курсором
        let point = Point::from((mouse_state.x(), mouse_state.y()));
        self.current_hover = GridPos::find_pointer_position(point);

        self.cursors.set(Cursor::Pointer);
        if let Some(pos) = self.current_hover {
            if let Some((side, unit)) = self.find_selected_unit(pos) {
                let hovered_part = pos.calculate_direction(point);
                dbg!((unit.creature, hovered_part));

                let cursor =
                    match hovered_part {
                        HexagonPart::Left         => Cursor::AttackLeft,
                        HexagonPart::Right        => Cursor::AttackRight,
                        HexagonPart::TopHalfLeft  => Cursor::AttackUpLeft,
                        HexagonPart::TopHalfRight => Cursor::AttackUpRight,
                        HexagonPart::BotHalfLeft  => Cursor::AttackDownLeft,
                        HexagonPart::BotHalfRight => Cursor::AttackDownRight,
                        HexagonPart::BotLeft      => Cursor::AttackDownLeft,
                        HexagonPart::BotRight     => Cursor::AttackDownRight,
                        HexagonPart::TopLeft      => Cursor::AttackUpLeft,
                        HexagonPart::TopRight     => Cursor::AttackUpRight,
                        _ => Cursor::Pointer
                    };
                self.cursors.set(cursor);
            }
        }

        // Выбираем тип курсора
        // let cursor =
        //     if is_unit_selected {
        //         Cursor::AttackLeft
        //     } else if self.current_hover.is_some() {
        //         Cursor::Run
        //     } else {
        //         Cursor::Pointer
        //     };
        // self.cursors.set(cursor);

        // Ловим конкретные события
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { 
                    std::process::exit(0)
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    let current_stack = self.get_current_stack_mut();
                    current_stack.defending = true;
                    current_stack.turn_state = CTS::NoTurn;
                    self.update_current_stack();
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        for side in &mut self.sides {
            for unit in side {
                unit.update(dt);
            }
        }
    }

    pub fn draw(
        &self,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>
    ) -> Result<(), Box<dyn Error>> {
        // Рисуем поле боя
        canvas.copy(&self.battlefield, None, Rect::new(0, 0, 800, 556))?;

        // Рисуем клетки на поле
        for x in GridPos::X_RANGE {
            for y in GridPos::Y_RANGE {
                let draw_rect = GridPos::new(x, y).bounding_rect();
                canvas.copy(&self.grid_cell, None, draw_rect)?;
            }
        }

        let pos = GridPos::new(6, 6);
        canvas.set_draw_color(Color::RED);
        // canvas.draw_rect(pos.draw_rect())?;
        canvas.draw_point(pos.center())?;

        let top_right = pos.center().offset((GridPos::CELL_WIDTH / 2) as i32, -((GridPos::CELL_VERTICAL / 2) as i32));
        canvas.draw_point(top_right)?;
        canvas.set_draw_color(Color::BLACK);

        // Выделяем клетку под курсором
        if let Some(pos) = &self.current_hover {
            canvas.copy(&self.grid_cell_shadow, None, pos.bounding_rect())?;
        }

        // Рисуем существ
        for side in &self.sides {
            for unit in side {
                unit.draw(canvas, rr, tc, false)?;
            }
        }

        let current_stack = self.get_current_stack();
        current_stack.draw(canvas, rr, tc, true)?;

        Ok(())
    }
}
