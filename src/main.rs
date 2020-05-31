extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod resources;
use resources::ResourceRegistry;
use resources::catalog::battlefields::{Battlefields, BATTLEFIELDS_GRAPHICS};

const LOD_ARCHIVE: &'static str = "/home/vsevolod/Wine/HoMM3/drive_c/HoMM3/Data/H3sprite.lod";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?; 
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rust", 800, 600)
        .position_centered()
        .build()?;
    
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()?;

    let mut resource_registry = ResourceRegistry::new(&canvas, &[&LOD_ARCHIVE]);
    // let texture = resource_registry.get_texture(LOD_ARCHIVE, BATTLEFIELDS_GRAPHICS[Battlefields::MAG as usize]);
    let animation = resource_registry.get_animation(LOD_ARCHIVE, "CCHAMP.def");
    dbg!(&animation.frames.keys());
    let texture = animation.frames.values().next().unwrap();
    let texture_query = texture.query();
    let texture_rect = sdl2::rect::Rect::new(0, 0, texture_query.width, texture_query.height);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => { 
                    break 'running 
                },
                _ => {}
            }
        }
        canvas.clear();
        canvas.copy(&texture, None, texture_rect)?;
        canvas.present();
    }
    Ok(())
}
