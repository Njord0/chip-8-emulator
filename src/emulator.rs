use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


const FONT: &[u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Proc {
    registers_g: [u8; 16],
    reg_i: u16,
    reg_dt: u8,
    reg_st: u16,
    pc: u16,
    sp: u8,
    memory: [u8; 4096],
    framebuffer: [u8; 64*32]
}

impl Proc {
    pub fn new() -> Self {
        Proc {
            registers_g: [0; 16],
            reg_i: 0,
            reg_dt: 0,
            reg_st: 0,
            pc: 0x200,
            sp: 0,
            memory: [0; 4096],
            framebuffer: [0; 64*32]
        }
    }

    pub fn next_instruction(&mut self) -> u16 {
        let a = u16::from_be_bytes([
            self.memory[self.pc as usize],
            self.memory[self.pc as usize + 1]]
        );
    
        self.pc += 2;

        a
    }

    pub fn load_program(&mut self, prog: &[u16]) {
        let mut i = 0x200;
        for j in prog {
            let a = j.to_be_bytes();   
            self.memory[i] = a[0];
            self.memory[i+1] = a[1];
            i += 2;
        }

        i = 0x50;
        // add font at beginning of memory
        for j in FONT {
            self.memory[i] = *j;
            i += 1;
        }

    }
    pub fn dump_regs(&self) {
        for i in 0..16 {
            println!("V{}: {}", i, self.registers_g[i]);
        }

        println!("I: {}", self.reg_i);
    }

    pub fn get_framebuffer(&self) -> &[u8] {&self.framebuffer}


    pub fn run(&mut self, event: &mut EventPump) {
        let i = self.next_instruction();

        match i & 0xF000 {
            0x0000 if i == 0x00E0 => {
                println!("CLS");
            },

            0x0000 if i == 0x00E0 => {
                println!("RET");
            },

            0x1000 => { // JP addr
                self.pc = i & 0x0FFF;
            },

            0x2000 => { // CALL addr
                self.memory[self.sp as usize] = self.pc.to_be_bytes()[0];
                self.memory[self.sp as usize +1] = self.pc.to_be_bytes()[1];

                self.sp += 2;

                self.pc = i & 0x0FFF;
            },

            0x3000 => { // SE Vx, byte
                let x = (i >> 8) & 0xF;
                let k = i & 0xFF;

                if u16::from(self.registers_g[x as usize]) == k {
                    self.pc += 2;
                }
            },

            0x4000 => { // SNE Vx, byte
                let x = (i >> 8) & 0xF;
                let k = i & 0xFF;
                if u16::from(self.registers_g[x as usize]) != k {
                    self.pc += 2;
                }
            },

            0x5000 => { // SE Vx, Vy
                let x = (i >> 8) & 0xF;
                let y = (i >> 4) & 0xF;

                if self.registers_g[x as usize] == self.registers_g[y as usize] {
                    self.pc += 2;
                }
            },

            0x6000 => { // LD Vx, byte
                let x = (i >> 8) & 0xF;
                let k = i & 0xFF;

                self.registers_g[x as usize] = k as u8;
            },

            0x7000 => { // ADD Vx, byte
                let x = (i >> 8) & 0xF;
                let k = i & 0xFF;

                self.registers_g[x as usize] += k as u8;
            },

            0x8000 => { // Some math here
                let x = (i >> 8) & 0xF;
                let y = (i >> 4) & 0xF;


                match i & 0xF {
                    0 => self.registers_g[x as usize] = self.registers_g[y as usize],
                    1 => {
                        self.registers_g[x as usize] = 
                            self.registers_g[y as usize] | self.registers_g[x as usize];
                    },

                    2 => {
                        self.registers_g[x as usize] = 
                            self.registers_g[y as usize] & self.registers_g[x as usize];
                    },
                    
                    3 => {
                        self.registers_g[x as usize] =
                            self.registers_g[y as usize] ^ self.registers_g[x as usize];
                    },

                    4 => {
                        if (self.registers_g[y as usize] + self.registers_g[x as usize]) as u16 > 255 {
                            self.registers_g[15] = 1;
                        }
                        else {
                            self.registers_g[15] = 0;
                        }
                        self.registers_g[x as usize] =
                            self.registers_g[y as usize] + self.registers_g[x as usize];                    
                    },

                    5 => {
                        if self.registers_g[x as usize] - self.registers_g[y as usize] > 0 {
                            self.registers_g[15] = 1;
                        }
                        else {
                            self.registers_g[15] = 0;
                        }

                        self.registers_g[x as usize] = 
                            self.registers_g[x as usize] - self.registers_g[y as usize];                    
                    },

                    6 => {
                        self.registers_g[15] = self.registers_g[x as usize] & 0x1;

                        self.registers_g[x as usize] = self.registers_g[x as usize] >> 1;
                    },

                    7 => {
                        if self.registers_g[y as usize] - self.registers_g[x as usize] > 0 {
                            self.registers_g[15] = 1;
                        }
                        else {
                            self.registers_g[15] = 0;
                        }

                        self.registers_g[x as usize] = 
                            self.registers_g[y as usize] - self.registers_g[x as usize];
                    },

                    0xE => {
                        self.registers_g[15] = 1;
                        self.registers_g[x as usize] = self.registers_g[x as usize] << 1;
                    }
                    _ => {}

                };
            },

            0x9000 => { // SNE Vx, Vy
                let x = (i >> 8) & 0xF;
                let y = (i >> 4) & 0xF;

                if self.registers_g[x as usize] != self.registers_g[y as usize] {
                    self.pc += 2;
                }

            },

            0xA000 => { // LD I, addr
                self.reg_i = i & 0xFFF;
            },

            0xB000 => { // JP V0, addr
                self.pc = self.reg_i + (i & 0xFFF);
            },

            0xC000 => { // RND Vx, byte
                // todo
            },

            0xD000 => { // DRW Vx, Vy, nibble
                let x = self.registers_g[((i >> 8) & 0x0F) as usize] as u16;
                let y = self.registers_g[((i >> 4) & 0x00F) as usize] as u16;
                let height = i & 0xF;

                self.registers_g[15] = 0;
                for yline in 0..height {
                    let pixel = self.memory[(self.reg_i + yline) as usize];
                    
                    for xline in 0..8 {
                        if pixel & (0x80 >> xline) != 0 {
                            self.framebuffer[(x+xline + ((y+yline)*64)) as usize] = 1;
                        }
                    }
                }
            },

            0xE000 => {
                match i & 0x00FF {
                    0x9E => { // SKP Vx
                        // todo
                    },

                    0xA1 => { //SNKP Vx

                    },
                    _ => {}
                };
            },

            0xF000 => {
                let x = (i >> 8) & 0xF;
                match i & 0x00FF {
                    0x07 => { // LD Vx, DT

                        self.registers_g[x as usize] = self.reg_dt;
                    },

                    0x0A => { // LD Vx, K
                        println!("Infinite loop here");
                        loop {
                            for event in event.poll_iter() {
                                match event {
                                    Event::KeyDown { keycode: Some(Keycode::Q), ..} => {return;},
                                    _ => {}
                                };
                            }
                        } // todo
                    },

                    0x15 => { // LD DT, Vx
                        self.reg_dt = self.registers_g[x as usize];
                    },

                    0x18 => { // LD ST, Vx
                        self.reg_st = self.registers_g[x as usize].into();
                    },

                    0x1E => { // ADD I, Vx
                        self.reg_i += self.registers_g[x as usize] as u16;
                    },

                    0x29 => { // LD F, Vx
                        // todo
                    },

                    0x33 => { // LD B, Vx
                        let y = self.registers_g[x as usize];
                        self.memory[self.reg_i as usize] = y / 100;
                        self.memory[self.reg_i as usize+1] = y / 10;
                        self.memory[self.reg_i as usize+2] = y;
                    },

                    0x55 => { // LD [i], Vx
                        for j in 0..16 {
                            self.memory[(self.reg_i+j) as usize] =
                                self.registers_g[j as usize];
                        }
                    },

                    0x65 => {
                        for j in 0..16 {
                            self.registers_g[j as usize] = 
                                self.memory[(self.reg_i+j) as usize];
                        }
                    },


                    _ => {}
                };
            },

            _ => {
                panic!("Unknow opcode: {:#04x}", i);
            },
        }

    }
}