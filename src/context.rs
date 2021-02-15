extern crate sdl2; 

pub struct Sdl {
    pub sdl_context: sdl2::Sdl,
    window_canvas: sdl2::render::WindowCanvas,
    pub timer: sdl2::TimerSubsystem,
}

impl Sdl {
    pub fn new(title: &str, width: u32, height: u32) -> Self {

        let sdl_context = match sdl2::init() {
            Ok(c) => c,
            Err(s) => panic!("{:?}", s),
        };

        let video = match sdl_context.video() {
            Ok(v) => v,
            Err(s) => panic!("{:?}", s)
        };

        let window = match video.window(title, width, height).build() {
            Ok(w) => w,
            Err(s) => panic!("{:?}", s),
        };

        let canvas = match window.into_canvas().build() {
            Ok(c) => c,
            Err(s) => panic!("{:?}", s)
        };

        let t = match sdl_context.timer() {
            Ok(t) => t,
            Err(s) => panic!("{:?}", s)
        };


        let a = Sdl {
            sdl_context: sdl_context,
            window_canvas: canvas,
            timer: t,
        };


        a
    }

    pub fn get_canvas(&mut self) -> &mut sdl2::render::WindowCanvas {
        &mut self.window_canvas
    }

    pub fn get_ticks(&mut self) -> u32 {
        self.timer.ticks()
    }

}