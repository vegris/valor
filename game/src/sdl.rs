use sdl2::render::WindowCanvas;

type Error = Box<dyn std::error::Error>;

pub struct Sdl {
    pub main_context: sdl2::Sdl,
    pub ttf_context: sdl2::ttf::Sdl2TtfContext,
    pub mixer_context: sdl2::mixer::Sdl2MixerContext,
}

impl Sdl {
    pub fn initialize() -> Result<Self, Error> {
        let main_context = sdl2::init()?;
        let ttf_context = sdl2::ttf::init()?;
        let mixer_context = crate::sound::initialize()?;

        Ok(Self {
            main_context,
            ttf_context,
            mixer_context,
        })
    }

    pub fn build_canvas(&self) -> Result<WindowCanvas, Error> {
        let video_subsystem = self.main_context.video()?;

        let window = video_subsystem
            .window("Rust", 800, 600)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().present_vsync().build()?;

        Ok(canvas)
    }
}
