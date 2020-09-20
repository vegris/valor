use std::time::Duration;

extern crate sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use itertools::iproduct;

use crate::enumerations::{Battlefield, Creature, Misc};
use crate::resources::ResourceRegistry;
use crate::util::AnyError;

use super::GridPos;
use super::creature::{CreatureStack, Direction};
use crate::graphics::animations::CreatureAnimation;
use crate::graphics::creature::AnimationType;
use crate::graphics::choreographer;
use crate::graphics::cursors::{Cursors, Cursor};

enum CommandType {
    Move,
    Attack
}

struct Command {
    type_: CommandType,
    destination: GridPos
}

impl Command {
    fn new(type_: CommandType, destination: GridPos) -> Self {
        Self { type_, destination }
    }
}

pub struct BattleState<'a> {
    // Постоянно используемые текстуры,
    // которые нет смысла прокачивать сквозь кэш
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,

    cursors: Cursors,

    current_hover: Option<GridPos>,
    pending_command: Option<Command>,

    creatures: Vec<CreatureStack>
}


impl<'a> BattleState<'a> {
    pub fn new(rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>, battlefield: Battlefield) -> Result<Self, AnyError> {
        let mut battlestate = Self {
            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency(Misc::CellGrid.filename())?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency(Misc::CellGridShadow.filename())?.as_texture(&tc)?,

            cursors: Cursors::load(rr),

            current_hover: None,
            pending_command: None,

            creatures: vec![
                CreatureStack::new(Creature::Enchanter, GridPos::new(5, 9), Direction::Right),
                CreatureStack::new(Creature::Peasant, GridPos::new(6, 9), Direction::Right)
            ]

        };

        battlestate.creatures[0].push_animation(CreatureAnimation::new_looping(AnimationType::Standing));

        Ok(battlestate)
    }

    pub fn process_input(&mut self, event_pump: &mut EventPump) {
        // Опрашиваем устройства
        event_pump.pump_events();

        // Получаем позицию мыши
        let mouse_state = event_pump.mouse_state();
        let point = (mouse_state.x(), mouse_state.y());

        // Текущая клетка под курсором
        self.current_hover = 
            iproduct!(GridPos::X_RANGE, GridPos::Y_RANGE)
                .map(|(x, y)| GridPos::new(x, y))
                .find(|pos| pos.contains_point(point));
        
        // Юнит под курсором
        let is_unit_selected = self.current_hover.and_then(|grid| {
            self.creatures.iter().find(|unit| unit.grid_pos() == grid)
        }).is_some();

        // Выбираем тип курсора
        let cursor =
            if is_unit_selected {
                Cursor::AttackLeft
            } else if self.current_hover.is_some() {
                Cursor::Run
            } else {
                Cursor::Pointer
            };
        self.cursors.set(cursor);

        // Ловим конкретные события
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { 
                    std::process::exit(0)
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, ..} => {
                    self.pending_command =
                        self.current_hover.map(|pos| {
                            let type_ =
                                if is_unit_selected {
                                    CommandType::Attack
                                } else {
                                    CommandType::Move
                                };
                            Command::new(type_, pos)
                        });
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {

        if let Some(Command { type_, destination }) = self.pending_command.take() {
            match type_ {
                CommandType::Move => {
                    // Команда "Двигаться"
                    let start_pos = self.creatures[0].grid_pos();
                    if let Some(path) = start_pos.get_shortest_path_to(destination) {
                        choreographer::animate_unit_move(self, rr, 0, &path);
                        let last_grid = path.last().unwrap();
                        self.creatures[0].set_grid_pos(*last_grid);
                    }
                },
                CommandType::Attack => {
                    // Команда "Атаковать"
                    choreographer::animate_melee_attack(self, rr, 0, 1);
                }
            };
        }

        for creature in &mut self.creatures {
            creature.update(dt);
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        // Рисуем поле боя
        canvas.copy(&self.battlefield, None, Rect::new(0, 0, 800, 556))?;

        // Рисуем клетки на поле
        for x in GridPos::X_RANGE {
            for y in GridPos::Y_RANGE {
                let draw_rect = GridPos::new(x, y).draw_rect();
                canvas.copy(&self.grid_cell, None, draw_rect)?;
            }
        }

        // Выделяем клетку под курсором
        if let Some(pos) = &self.current_hover {
            canvas.copy(&self.grid_cell_shadow, None, pos.draw_rect())?;
        }

        // Рисуем существ
        for creature in &self.creatures {
            creature.draw(canvas, rr, tc)?;
        }

        Ok(())
    }

    pub fn get_unit_mut(&mut self, unit_index: usize) -> &mut CreatureStack {
        &mut self.creatures[unit_index]
    }
}
