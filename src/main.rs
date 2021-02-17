extern crate sdl2;

mod context;
mod emulator; 

use emulator::Proc;

use std::fs::File;
use std::io::prelude::*;


fn get_file_content(mut file: File) -> Result<Vec<u16>, String> {

    let mut buffer = Vec::<u8>::new();

    if let Ok(_) = file.read_to_end(&mut buffer) {
        let buff: Vec<u16> = buffer
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_be_bytes([a[0], a[1]]))
            .collect();

        return Ok(buff);
    }

    Err("Error while reading file".to_owned())


}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("I need a file to feed the emulator :'(");
        return;
    }
    
    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(s) => {
            println!("Error while opening file: {:?}", s);
            return;
        } 
    };

    let a = get_file_content(file).unwrap();

    let mut proc = Proc::new();

    proc.load_program(&a);

    let mut context = context::Sdl::new("chip-8", 64*8, 32*8);
    let mut event_pump = context.sdl_context.event_pump().unwrap();

    loop {
        
        proc.update_key(&mut event_pump);
        proc.run(&mut event_pump);

        let a = proc.get_framebuffer();
        {
            let canvas = context.get_canvas();
            canvas.clear();

            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

            for i in 0..64 {
                for j in 0..32 {
                    if a[(j*64) + i] == 1 {
                        if let Err(s) = canvas.fill_rect(sdl2::rect::Rect::new((i * 8) as i32, (j*8) as i32, 8, 8)) {
                            println!("{:?}", s);
                        }
     
                    }
                }
            }
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

            canvas.present();
        }

        if context.get_ticks() - proc.timers_last > 1000/60 {
            proc.dec_timers();
            proc.timers_last = context.get_ticks();
        }

        
    }
}