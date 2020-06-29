use std::time::Instant;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod resources;
use resources::ResourceRegistry;

mod enumerations;
use enumerations::Battlefield;

mod gamestates;
use gamestates::BattleState;

mod util;
use util::AnyError;


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
    let mut current_state = BattleState::new(Battlefield::CUR);

    let mut frame_start_time = Instant::now();
    'gameloop: loop {
        // Обработка ввода
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { 
                    break 'gameloop 
                },
                _ => {}
            }
        }
        // Обновление игрового состояния
        let dt = frame_start_time.elapsed();
        frame_start_time = Instant::now();
        current_state.update(dt);

        // Отображение игрового состояния
        canvas.clear();
        current_state.draw(&mut canvas, &mut resource_registry, &texture_creator)?;
        canvas.present();
    }

    Ok(())
}
