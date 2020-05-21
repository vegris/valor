extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod resources;
use resources::ResourceRegistry;


fn main() {

    let sdl_context = sdl2::init().unwrap(); 
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Rust", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let LOD_ARCHIVE = String::from("/home/vsevolod/Wine/HoMM3/drive_c/HoMM3/Data/H3bitmap.lod");

    let mut resource_registry = ResourceRegistry::new(&canvas, &[&LOD_ARCHIVE]);
    let texture = resource_registry.get_texture(&LOD_ARCHIVE, &"CmBkBoat.pcx".to_string());
    let texture_query = texture.query();
    let texture_rect = sdl2::rect::Rect::new(0, 0, texture_query.width, texture_query.height);

    let mut event_pump = sdl_context.event_pump().unwrap();
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
        canvas.copy(&texture, None, texture_rect).unwrap();
        canvas.present();
    }
}
