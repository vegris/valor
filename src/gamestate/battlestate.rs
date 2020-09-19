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

struct Command {
    destination: GridPos
}

impl Command {
    fn new(destination: GridPos) -> Self {
        Self { destination }
    }
}

pub struct BattleState<'a> {
    // Постоянно используемые текстуры,
    // которые нет смысла прокачивать сквозь кэш
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,

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

        self.current_hover = 
            iproduct!(GridPos::X_RANGE, GridPos::Y_RANGE)
                .map(|(x, y)| GridPos::new(x, y))
                .find(|pos| pos.contains_point(point));

        // Ловим конкретные события
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { 
                    std::process::exit(0)
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, ..} => {
                    // По клику мыши создать команду существу двигаться в указанную клетку
                    if let Some(pos) = self.current_hover {
                        self.pending_command = Some(Command::new(pos));
                    }
                },
                _ => {}
            }
        }
    }

    pub fn update(&mut self, dt: Duration, rr: &mut ResourceRegistry) {
        if let Some(command) = self.pending_command.take() {
            // Команда "Двигаться"
            // dbg!(command.destination);
            // let start_pos = self.creatures[0].grid_pos();
            // if let Some(path) = start_pos.get_shortest_path_to(command.destination) {
            //     choreographer::animate_unit_move(self, rr, 0, &path);
            //     let last_grid = path.last().unwrap();
            //     self.creatures[0].set_grid_pos(*last_grid);
            // }

            // Команда "Атаковать"
            choreographer::animate_melee_attack(self, rr, 0, 1);
        }

        for creature in &mut self.creatures {
            creature.update(dt);
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry, tc: &TextureCreator<WindowContext>) -> Result<(), AnyError> {
        // Рисуем поле боя
        canvas.copy(&self.battlefield, None, Rect::new(0, 0, 800, 556))?;
        // Рисуем сетку
        self.draw_grid(canvas)?;

        // Рисуем существ
        for creature in &self.creatures {
            creature.draw(canvas, rr, tc)?;
        }

        Ok(())
    }

    pub fn get_unit_mut(&mut self, unit_index: usize) -> &mut CreatureStack {
        &mut self.creatures[unit_index]
    }

    fn draw_grid(&self, canvas: &mut WindowCanvas) -> Result<(), AnyError> {
        for x in GridPos::X_RANGE {
            for y in GridPos::Y_RANGE {
                let draw_rect = GridPos::new(x, y).draw_rect();
                canvas.copy(&self.grid_cell, None, draw_rect)?;
            }
        }
        if let Some(pos) = &self.current_hover {
            canvas.copy(&self.grid_cell_shadow, None, pos.draw_rect())?;
        }
        Ok(())
    }
}
