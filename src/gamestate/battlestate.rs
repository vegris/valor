use std::time::Duration;

extern crate sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator, Texture};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;

use itertools::iproduct;

use crate::enumerations::{Battlefield, Creature, Misc};
use crate::resources::ResourceRegistry;
use crate::util::AnyError;

use super::GridPos;
use super::creature::CreatureStack;
use crate::graphics::animations::CreatureAnimation;
use crate::graphics::choreographer;
use crate::graphics::creature::AnimationType;

pub struct BattleState<'a> {
    // Постоянно используемые текстуры,
    // которые нет смысла прокачивать сквозь кэш
    battlefield: Texture<'a>,
    grid_cell: Texture<'a>,
    grid_cell_shadow: Texture<'a>,

    current_hover: Option<GridPos>,

    creatures: Vec<CreatureStack>
}


impl<'a> BattleState<'a> {
    pub fn new(rr: &mut ResourceRegistry, tc: &'a TextureCreator<WindowContext>, battlefield: Battlefield) -> Result<Self, AnyError> {
        let mut battlestate = Self {
            battlefield: rr.load_pcx(battlefield.filename())?.as_texture(&tc)?,
            grid_cell: rr.load_pcx_with_transparency(Misc::CellGrid.filename())?.as_texture(&tc)?,
            grid_cell_shadow: rr.load_pcx_with_transparency(Misc::CellGridShadow.filename())?.as_texture(&tc)?,

            current_hover: None,

            creatures: vec![
                CreatureStack::new(Creature::Champion, GridPos::new(5, 9), true),
                CreatureStack::new(Creature::Peasant, GridPos::new(7, 9), false)
            ]

        };

        battlestate.creatures[0].push_animation(CreatureAnimation::new(AnimationType::Standing));
        battlestate.creatures[0].push_animation(CreatureAnimation::new(AnimationType::Moving));
        battlestate.creatures[0].push_animation(CreatureAnimation::new(AnimationType::Moving));
        battlestate.creatures[0].push_animation(CreatureAnimation::new(AnimationType::Moving));
        battlestate.creatures[0].push_animation(CreatureAnimation::new(AnimationType::Moving));
        battlestate.creatures[0].push_animation(CreatureAnimation::new_turning(AnimationType::TurnLeft));
        battlestate.creatures[0].push_animation(CreatureAnimation::new(AnimationType::TurnRight));
        battlestate.creatures[0].push_animation(CreatureAnimation::new_looping(AnimationType::Standing));

        // battlestate.creatures[0].push_animation(CreatureAnimation::new_looping(AnimationType::Standing));
        // battlestate.creatures[1].push_animation(CreatureAnimation::new_looping(AnimationType::Standing));

        // battlestate.creatures[0].push_animation(CreatureAnimation::new_turning(AnimationType::AttackStraight));
        // battlestate.creatures[1].push_animation(CreatureAnimation::new_delayed(AnimationType::GettingHit, Duration::from_millis(512)));

        // battlestate.creatures[0].push_animation(CreatureAnimation::new_looping(AnimationType::Standing));
        // battlestate.creatures[1].push_animation(CreatureAnimation::new_looping(AnimationType::Standing));

        // let champion_path = vec![
        //     GridPos::new(6, 9),
        //     GridPos::new(7, 9),
        //     GridPos::new(8, 9),
        //     GridPos::new(9, 9),
        //     GridPos::new(10, 9),
        //     GridPos::new(11, 9),
        //     GridPos::new(12, 9),
        //     GridPos::new(13, 9),
        // ];
        // choreographer::animate_unit_move(&mut battlestate, rr, 0, champion_path);
        // choreographer::animate_unit_standing(&mut battlestate, rr, 0, Instant::now());

        Ok(battlestate)
    }

    pub fn process_input(&mut self, event_pump: &mut EventPump) {
        // Обязательный поллинг событий
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { 
                    std::process::exit(0)
                },
                _ => {}
            }
        }

        let mouse_state = event_pump.mouse_state();
        let point = (mouse_state.x(), mouse_state.y());

        self.current_hover = 
            iproduct!(GridPos::X_RANGE, GridPos::Y_RANGE)
                .map(|(x, y)| GridPos::new(x, y))
                .find(|pos| pos.contains_point(point));
    }

    pub fn update(&mut self, dt: Duration) {
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
