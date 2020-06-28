use std::time::{Duration, Instant};

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod resources;
use resources::{ResourceRegistry, DefContainer};
use resources::battlefields::Battlefield;
use resources::creatures::{Creature, AnimationGroup};

struct BattleStateGraphics<'a> {
    battlefield: Texture<'a>,
    creature: DefContainer
}
struct BattleState<'a> {
    battlefield: Battlefield,
    creature_stance: (Duration, Texture<'a>),
    graphics: BattleStateGraphics<'a>
}

impl<'a> BattleState<'a> {
    fn new<'b>(battlefield: Battlefield, creature: Creature, tc: &'a TextureCreator<WindowContext>, rr: &'b mut ResourceRegistry) -> BattleState<'a> {
        let battlefield_graphics = rr.load_pcx(battlefield.filename()).as_texture(tc).unwrap();

        let mut creature_graphics = rr.load_def(creature.filename());
        let initial_creature_sprite = creature_graphics.get_sprite_for_animation(AnimationGroup::Moving, 0.0);
        let initial_creature_texture = initial_creature_sprite.surface.as_texture(tc).unwrap();

        BattleState {
            battlefield,
            creature_stance: (Duration::new(0, 0), initial_creature_texture),
            graphics: BattleStateGraphics {
                battlefield: battlefield_graphics,
                creature: creature_graphics
            }
        }
    }

    fn update(&mut self, dt: Duration, tc: &'a TextureCreator<WindowContext>) {
        const update_speed: Duration = Duration::from_secs(1);
        let (mut duration, texture) = &self.creature_stance;
        duration = duration + dt;
        if duration >= update_speed {
            duration = duration - update_speed;
        }

        let percent = duration.as_millis() as f32 / update_speed.as_millis() as f32;
        let sprite = self.graphics.creature.get_sprite_for_animation(AnimationGroup::Death, percent);
        let texture = sprite.surface.as_texture(tc).unwrap();

        self.creature_stance = (duration, texture)
    }

    fn render(&self, canvas: &mut WindowCanvas, _rr: &mut ResourceRegistry) -> Result<(), String> {
        canvas.copy(&self.graphics.battlefield, None, Rect::new(0, 0, 800, 556))?;
        let (_, creature_texture) = &self.creature_stance;
        canvas.copy(&creature_texture, None, Rect::new(256, 256, 80, 80))?;
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
        current_state.update(dt, &texture_creator);

        // Отображение игрового состояния
        canvas.clear();
        current_state.render(&mut canvas, &mut resource_registry)?;
        canvas.present();
    }
    Ok(())
}
