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
    registers_g: [u8; 16], // general purpose registers
    reg_i: u16,            // I register
    reg_dt: u8,            // delay timer
    reg_st: u16,           // sound timer
    pc: u16,               // program counter
    sp: u16,                // stack pointer
    memory: [u8; 4096],
    framebuffer: [u8; 64*32],
    key: [u8; 16],
    pub timers_last: u32
}

impl Proc {
    pub fn new() -> Self {
        Proc {
            registers_g: [0; 16],
            reg_i: 0,
            reg_dt: 0,
            reg_st: 0,
            pc: 0x200,
            sp: 4000,
            memory: [0; 4096],
            framebuffer: [0; 64*32],
            key: [0; 16],
            timers_last: 0
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

    pub fn update_key(&mut self, event: &mut EventPump) {
        for event in event.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(x), ..} => {
                    match x {
                        Keycode::Num1 => self.key[0] = 1,
                        Keycode::Num2 => self.key[1] = 1,
                        Keycode::Num3 => self.key[2] = 1, 
                        Keycode::Num4 => self.key[3] = 1,
                        Keycode::A => self.key[4] = 1,
                        Keycode::Z => self.key[5] = 1,
                        Keycode::E => self.key[6] = 1,
                        Keycode::R => self.key[7] = 1,
                        Keycode::Q => self.key[8] = 1,
                        Keycode::S => self.key[9] = 1,
                        Keycode::D => self.key[10] = 1,
                        Keycode::F => self.key[11] = 1,
                        Keycode::W => self.key[12] = 1,
                        Keycode::X => self.key[13] = 1,
                        Keycode::C => self.key[14] = 1,
                        Keycode::V => self.key[15] = 1,

                        Keycode::Escape => panic!("exit"),

                    _ => {} 
                    };
                },
                _ => {}
            };

        }
    }

    pub fn dec_timers(&mut self) {
        if self.reg_dt > 0 {
            self.reg_dt -= 1;
        }

        if self.reg_st > 0 {
            self.reg_st -= 1;
        }
    }


    pub fn run(&mut self, event: &mut EventPump) {
        let i = self.next_instruction();

        match i & 0xF000 { 
            0x0000 if i == 0x00E0 => { // CLS
                for i in 0..2048 {
                    self.framebuffer[i] = 0;
                }
            },

            0x0000 if i == 0x00EE => { // RET
                self.pc = u16::from_be_bytes([self.memory[(self.sp) as usize],
                                            self.memory[(self.sp-1) as usize]]);
                self.sp -= 2;
            },

            0x1000 => { // JP addr
                self.pc = i & 0x0FFF;
            },

            0x2000 => { // CALL addr
                self.sp += 2;
                self.memory[self.sp as usize] = self.pc.to_be_bytes()[0];
                self.memory[self.sp as usize -1] = self.pc.to_be_bytes()[1];

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

                let result = self.registers_g[x as usize] as u16 + k;

                self.registers_g[x as usize] = result as u8;
            },

            0x8000 => { // Some math here
                let x = (i >> 8) & 0xF;
                let y = (i >> 4) & 0xF;

                match i & 0xF {
                    0 => self.registers_g[x as usize] = self.registers_g[y as usize],
                    1 => {
                        self.registers_g[x as usize] = 
                            self.registers_g[x as usize] | self.registers_g[y as usize];
                    },

                    2 => {
                        self.registers_g[x as usize] = 
                            self.registers_g[x as usize] & self.registers_g[y as usize];
                    },
                    
                    3 => {
                        self.registers_g[x as usize] =
                            self.registers_g[x as usize] ^ self.registers_g[y as usize];
                    },

                    4 => {
                        let result = self.registers_g[x as usize] as u16 + self.registers_g[y as usize] as u16;

                        if result > 255 {
                            self.registers_g[15] = 1;
                        }     
                        else {
                            self.registers_g[15] = 0;
                        }

                        self.registers_g[x as usize] = result as u8;
                    },

                    5 => {

                        if self.registers_g[x as usize] > self.registers_g[y as usize] {
                            self.registers_g[15] = 1;
                        }
                        else {
                            self.registers_g[15] = 0;
                        }
                        self.registers_g[x as usize] =
                            self.registers_g[x as usize].wrapping_sub(self.registers_g[y as usize]);
                    },

                    6 => {
                        self.registers_g[15] = self.registers_g[x as usize] & 0x1;

                        self.registers_g[x as usize] = self.registers_g[x as usize] >> 1;
                    },

                    7 => {
                        if self.registers_g[y as usize] > self.registers_g[x as usize] {
                            self.registers_g[15] = 1;
                        }
                        else {
                            self.registers_g[15] = 0;
                        }

                        self.registers_g[x as usize] = 
                            self.registers_g[y as usize].wrapping_sub(self.registers_g[x as usize]);
                    },

                    0xE => {
                        self.registers_g[15] = (self.registers_g[x as usize] >> 7) & 0xF;
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
                let y = self.registers_g[((i >> 4) & 0x0F) as usize] as u16;
                let height = i & 0xF;

                self.registers_g[15] = 0;
                for yline in 0..height {
                    let pixel = self.memory[(self.reg_i + yline) as usize];
                    
                    for xline in 0..8 {
                        if pixel & (0x80 >> xline) != 0 {
                            if x+xline + (y+yline)*64 < 2048 {
                                self.framebuffer[(x+xline + (y+yline)*64) as usize] ^= 1;
                                if self.framebuffer[(x+xline + (y+yline)*64) as usize] == 0 {
                                    self.registers_g[15] = 1;
                                }
                            }
                        }
                    }
                }
            },

            0xE000 => {
                let x = (i >> 8) & 0xF;
                match i & 0x00FF {
                    0x9E => { // SKP Vx
                        self.update_key(event);
                        if self.key[x as usize] == 1 {
                            self.pc += 2;
                            self.key[x as usize] = 0;
                        }

                    },

                    0xA1 => { //SNKP Vx
                        self.update_key(event);
                        if self.key[x as usize] != 1 {
                            self.pc += 2;
                        }
                        self.key[x as usize] = 0;

                    },
                    _ => {}
                };
            },

            0xF000 => {
                let x = (i >> 8) & 0xF;
                match i & 0x00FF {
                    0x07 => { // LD Vx, DT

                        self.registers_g[x as usize] = self.reg_dt.into();
                    },

                    0x0A => { // LD Vx, K
                        loop {
                            for event in event.poll_iter() {
                                match event {
                                    Event::KeyDown { keycode: Some(y), ..} => {
                                        match y {
                                            Keycode::Num1 => 
                                                self.registers_g[x as usize] = 0,
                                            Keycode::Num2 => 
                                                self.registers_g[x as usize] = 1,
                                            Keycode::Num3 => 
                                                self.registers_g[x as usize] = 2,
                                            Keycode::Num4 => 
                                                self.registers_g[x as usize] = 3,
                                            Keycode::A => 
                                                self.registers_g[x as usize] = 4,
                                            Keycode::Z => 
                                                self.registers_g[x as usize] = 5,
                                            Keycode::E => 
                                                self.registers_g[x as usize] = 6,
                                            Keycode::R => 
                                                self.registers_g[x as usize] = 7,
                                            Keycode::Q => 
                                                self.registers_g[x as usize] = 8,
                                            Keycode::S => 
                                                self.registers_g[x as usize] = 9,
                                            Keycode::D => 
                                                self.registers_g[x as usize] = 10,
                                            Keycode::F => 
                                                self.registers_g[x as usize] = 11,
                                            Keycode::W => 
                                                self.registers_g[x as usize] = 12,
                                            Keycode::X => 
                                                self.registers_g[x as usize] = 13,
                                            Keycode::C => 
                                                self.registers_g[x as usize] = 14,
                                            Keycode::V => 
                                                self.registers_g[x as usize] = 15,
                                                                                        
                                            _ => {}
                                        };

                                        return;
                                    },
                                    _ => {}
                                };
                            }
                        } // todo
                    },

                    0x15 => { // LD DT, Vx
                        self.reg_dt = self.registers_g[x as usize] as u8;
                    },

                    0x18 => { // LD ST, Vx
                        self.reg_st = self.registers_g[x as usize].into();
                    },

                    0x1E => { // ADD I, Vx
                        self.reg_i += self.registers_g[x as usize] as u16;
                    },

                    0x29 => { // LD F, Vx
                        self.reg_i = (self.registers_g[x as usize] * 5 + 0x50).into();
                    },

                    0x33 => { // LD B, Vx
                        let y = self.registers_g[x as usize];
                        self.memory[self.reg_i as usize] = (y / 100) as u8;
                        self.memory[self.reg_i as usize+1] = (y / 10) as u8;
                        self.memory[self.reg_i as usize+2] = y as u8;
                    },

                    0x55 => { // LD [i], Vx
                        for j in 0..x {
                            self.memory[(self.reg_i+j) as usize] =
                                self.registers_g[j as usize] as u8;
                        }
                    },

                    0x65 => { // LD Vx [I]
                        for j in 0..x {
                            self.registers_g[j as usize] = 
                                self.memory[(self.reg_i+j) as usize];
                        }
                    },


                    _ => {}
                };
            },

            _ => {
                println!("Unknow opcode: {:#04x}", i);
                loop {

                }
            },
        }

    }
}