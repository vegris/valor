extern crate sdl2;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};

use crate::creature_stack::CreatureTurnState as CTS;
use crate::gridpos::{GridPos, HexagonPart};
use crate::graphics::cursors::Cursor;

use super::BattleState;

impl<'a> BattleState<'a> {
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
}
