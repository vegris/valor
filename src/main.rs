use std::time::{Duration, Instant};

extern crate sdl2;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod resources;
use resources::{ResourceRegistry, RawPcx, RawDef};
use resources::battlefields::{Battlefields, BATTLEFIELDS_GRAPHICS};

struct BattleState {
    battlefield: Battlefields,
    battlefield_graphics: RawPcx,
    unit_spritesheet: RawDef
}

impl BattleState {
    fn update(&mut self, dt: Duration) {

    }

    fn render(&mut self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry) {
        let surface = self.battlefield_graphics.construct_surface();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();
        canvas.copy(&texture, None, None);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?; 

    // Инициализация видео подсистемы
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust", 800, 600)
        .position_centered()
        .build()?;
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()?;

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    // Создание начального игрового состояния
    let mut current_state = BattleState {
        battlefield: Battlefields::BOAT,
        battlefield_graphics: resource_registry.read_pcx_data(BATTLEFIELDS_GRAPHICS[Battlefields::BOAT as usize]),
        unit_spritesheet: resource_registry.read_def_data("CCHAMP.def")
    };

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
        current_state.render(&mut canvas, &mut resource_registry);
        canvas.present();
    }
    Ok(())
}
