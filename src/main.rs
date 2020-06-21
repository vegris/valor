#![allow(warnings)]
use std::time::{Duration, Instant};

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod resources;
use resources::{ResourceRegistry, Animation};
use resources::battlefields::Battlefield;
use resources::creatures::Creature;

struct BattleStateGraphics<'a> {
    battlefield: Texture<'a>,
    creature: Animation<'a>
}
struct BattleState<'a> {
    battlefield: Battlefield,
    creature_stance: (Duration, usize),
    graphics: BattleStateGraphics<'a>
}

impl<'a> BattleState<'a> {
    fn new<'b>(battlefield: Battlefield, creature: Creature, tc: &'a TextureCreator<WindowContext>, rr: &'b mut ResourceRegistry) -> BattleState<'a> {
        let battlefield_graphics = rr.read_pcx_data(battlefield.filename()).to_texture(tc);

        let creature_graphics = rr.read_def_data(creature.filename()).to_animation(tc);

        BattleState {
            battlefield,
            creature_stance: (Duration::new(0, 0), 0),
            graphics: BattleStateGraphics {
                battlefield: battlefield_graphics,
                creature: creature_graphics
            }
        }
    }

    fn update(&mut self, dt: Duration) {
        let update_speed = Duration::from_millis(256);
        let (mut duration, mut index) = self.creature_stance;
        duration = duration + dt;
        if duration >= update_speed {
            index = (index + 1) % self.graphics.creature.blocks2indexes.get(&1).unwrap().len();
            dbg!(index);
            duration = duration - update_speed
        }
        self.creature_stance = (duration, index)
    }

    fn render(&self, canvas: &mut WindowCanvas, rr: &mut ResourceRegistry) -> Result<(), String> {
        canvas.copy(&self.graphics.battlefield, None, Rect::new(0, 0, 800, 556))?;
        let (_, index) = self.creature_stance;
        let texture_index = self.graphics.creature.blocks2indexes.get(&1).unwrap()[index];
        let creature_texture = &self.graphics.creature.textures[texture_index];
        canvas.copy(&creature_texture, None, Rect::new(0, 0, 256, 256))?;
        Ok(())
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
    let texture_creator = canvas.texture_creator();

    // Открытие файлов с ресурсами
    let mut resource_registry = ResourceRegistry::init();

    // Инициализация подсистемы событий
    let mut event_pump = sdl_context.event_pump()?;

    // Создание начального игрового состояния
    let mut current_state = BattleState::new(Battlefield::CUR, Creature::Peasant, &texture_creator, &mut resource_registry);

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
        current_state.render(&mut canvas, &mut resource_registry)?;
        canvas.present();
    }
    Ok(())
}
