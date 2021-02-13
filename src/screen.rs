extern crate sdl2; 

pub struct Display {
    pub sdl_context: sdl2::Sdl,
    window_canvas: sdl2::render::WindowCanvas,
}

impl Display {
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

        let a = Display {
            sdl_context: sdl_context,
            window_canvas: canvas,
        };

        a
    }

    pub fn get_canvas(&mut self) -> &mut sdl2::render::WindowCanvas {
        &mut self.window_canvas
    }

}