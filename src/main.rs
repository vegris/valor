use std::time::Instant;

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

mod resources;
use resources::ResourceRegistry;

mod enumerations;
use enumerations::Battlefield;

mod gamestate;
use gamestate::BattleState;

mod util;
use util::AnyError;

mod graphics;
use graphics::cursors::{Cursor, Cursors};


fn main() -> Result<(), AnyError> {
    let sdl_context = sdl2::init()?; 

    // Инициализация видео подсистемы
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust", 800, 600)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()?;
    let texture_creator = canvas.texture_creator();

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    // Создание начального игрового состояния
    let mut current_state = BattleState::new(&mut resource_registry, &texture_creator, Battlefield::GRMT)?;

    // Загрузка и установка курсоров
    let cursors = Cursors::load(&mut resource_registry);
    cursors.set(Cursor::AttackLeft);

    let mut last_time = Instant::now();
    loop {
        let current_time = Instant::now();
        // Обработка ввода
        current_state.process_input(&mut event_pump);

        // Обновление игрового состояния
        current_state.update(current_time - last_time);
        
        let mouse_state = event_pump.mouse_state();
        let cursor_rect = Rect::from_center((mouse_state.x(), mouse_state.y()), 10, 10);

        // Отображение игрового состояния
        canvas.clear();
        current_state.draw(&mut canvas, &mut resource_registry, &texture_creator)?;

        canvas.set_draw_color(Color::RED);
        canvas.draw_rect(cursor_rect)?;
        canvas.set_draw_color(Color::BLACK);

        canvas.present();

        last_time = current_time;
    }
}
