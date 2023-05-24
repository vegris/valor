use std::error::Error;

pub struct Context {
    sdl: sdl2::Sdl,
    ttf: sdl2::ttf::Sdl2TtfContext,
}

impl Context {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let sdl = sdl2::init()?;
        let ttf = sdl2::ttf::init()?;

        Ok(Self { sdl, ttf })
    }

    pub fn canvas(&self) -> Result<sdl2::render::WindowCanvas, Box<dyn Error>> {
        let video_subsystem = self.sdl.video()?;

        let window = video_subsystem
            .window("Rust", 800, 600)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().present_vsync().build()?;

        Ok(canvas)
    }

    pub fn event_pump(&self) -> Result<sdl2::EventPump, Box<dyn Error>> {
        let event_pump = self.sdl.event_pump()?;
        Ok(event_pump)
    }

    pub fn load_font<'a>(
        &'a self,
        path: &str,
        size: u16,
    ) -> Result<sdl2::ttf::Font<'a, 'static>, Box<dyn Error>> {
        let font = self.ttf.load_font(path, size)?;
        Ok(font)
    }
}
