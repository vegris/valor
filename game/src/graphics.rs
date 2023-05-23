use std::error::Error;

use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::battlestate::BattleState;
use crate::command::Command;
use crate::config::Config;
use crate::grid::GridPos;
use crate::input::FrameData;
use crate::pathfinding;
use crate::registry::ResourceRegistry;

pub mod creature;
mod cursors;
pub mod stack;

use cursors::{Cursor, Cursors};

pub struct Graphics<'a> {
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,
    stack_count_bg: Texture<'a>,

    cursors: Cursors,
}

impl<'a> Graphics<'a> {
    pub fn init(
        config: &Config,
        rr: &mut ResourceRegistry,
        tc: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, Box<dyn Error>> {
        let [battlefield, grid_cell, grid_cell_shadow, stack_count_bg]: [Texture; 4] = [
            (config.battlefield.filename(), false),
            ("CCellGrd.pcx", true),
            ("CCellShd.pcx", true),
            ("CmNumWin.pcx", false),
        ]
        .into_iter()
        .map(|(filename, with_transparency)| {
            let surface = if with_transparency {
                rr.load_pcx_with_transparency(filename)
            } else {
                rr.load_pcx(filename)
            }?;

            let texture = surface.as_texture(tc)?;

            Ok(texture)
        })
        .collect::<Result<Vec<Texture>, Box<dyn Error>>>()?
        .try_into()
        .ok()
        .unwrap();

        let graphics = Graphics {
            battlefield,
            grid_cell,
            grid_cell_shadow,
            stack_count_bg,

            cursors: Cursors::load(rr),
        };
        Ok(graphics)
    }

    pub fn draw(
        &self,
        state: &BattleState,
        frame_data: FrameData,
        canvas: &mut WindowCanvas,
        rr: &mut ResourceRegistry,
        tc: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), Box<dyn Error>> {
        // Рисуем поле боя
        canvas.copy(
            &self.battlefield,
            None,
            sdl2::rect::Rect::new(0, 0, 800, 556),
        )?;

        // Рисуем клетки на поле
        for x in GridPos::X_RANGE {
            for y in GridPos::Y_RANGE {
                let draw_rect = GridPos::new(x, y).bounding_rect();
                canvas.copy(&self.grid_cell, None, draw_rect)?;
            }
        }

        let mut highlighted_cells = vec![];

        // Выделяем клетку под курсором
        if let Some(cell) = frame_data.current_hover {
            highlighted_cells.push(cell);
        }

        // Выставляем курсор под ситуацию
        let cursor = choose_cursor(state, &frame_data);
        self.cursors.set(cursor);

        if let Some(command) = frame_data.potential_lmb_command {
            match command {
                // Выделяем потенциальную позицию атакующего стека в случае атаки
                Command::Attack {
                    attack_position,
                    attack_direction,
                } => {
                    let current_stack = state.get_current_stack();
                    let current_side = current_stack.side;

                    let potential_position = pathfinding::unit_position_for_attack(
                        attack_position,
                        attack_direction,
                        current_side,
                        current_stack.creature.is_wide(),
                    );

                    if let Some(pos) = potential_position {
                        let occupied_cells = pathfinding::get_occupied_cells_for(
                            current_stack.creature,
                            current_side,
                            pos,
                        );

                        if let Some(cells) = occupied_cells {
                            highlighted_cells.extend(cells)
                        }

                        let handle = state.find_unit_for_cell(attack_position).unwrap();
                        let target_creature = state.get_stack(handle);

                        for cell in target_creature.get_occupied_cells() {
                            highlighted_cells.push(cell);
                        }
                    }
                }
                // Выделяем потенциальную позицию после перемещения (объединить в функцию с верхней)
                Command::Move { destination } => {
                    let current_stack = state.get_current_stack();

                    let occupied_cells = pathfinding::get_occupied_cells_for(
                        current_stack.creature,
                        current_stack.side,
                        destination,
                    );

                    if let Some(cells) = occupied_cells {
                        highlighted_cells.extend(cells);
                    }
                }
                _ => {}
            }
        }

        for cell in &state.reachable_cells {
            canvas.copy(&self.grid_cell_shadow, None, cell.bounding_rect())?;
        }

        highlighted_cells.sort();
        highlighted_cells.dedup();
        for cell in highlighted_cells {
            canvas.copy(&self.grid_cell_shadow, None, cell.bounding_rect())?;
        }

        // Рисуем существ
        let mut units = state.units();
        units.sort_unstable_by_key(|&handle| state.get_stack(handle).head.y);

        for handle in units {
            let is_current = handle == state.current_stack;
            let stack = state.get_stack(handle);
            stack::draw(
                stack,
                canvas,
                rr,
                tc,
                is_current,
                &self.stack_count_bg,
                font,
            )?;
            canvas.set_draw_color(Color::RED);
            canvas.draw_rect(stack.head.bounding_rect())?;
        }

        canvas.set_draw_color(Color::BLACK);

        Ok(())
    }
}

fn choose_cursor(state: &BattleState, frame_data: &FrameData) -> Cursor {
    let current_stack = state.get_current_stack();

    if let Some(command) = frame_data.potential_lmb_command {
        match command {
            Command::Move { .. } => {
                if current_stack.creature.is_flying() {
                    Cursor::Fly
                } else {
                    Cursor::Run
                }
            }
            Command::Attack {
                attack_direction, ..
            } => Cursor::from_attack_direction(attack_direction),
            Command::Shoot { .. } => Cursor::Arrow,
            _ => unreachable!(),
        }
    } else {
        Cursor::Pointer
    }
}
