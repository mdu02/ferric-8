use crate::ram::RAM;
use crate::byte_register::ByteRegister;
use crate::word_register::WordRegister;
use crate::graphics_buffer::GraphicsBuffer;
use rand;
use rand::Rng;

pub struct CPU {
    v: [ByteRegister; 16],
    index: WordRegister,
    program_counter: WordRegister,
    stack: [WordRegister; 16],
    stack_pointer: ByteRegister,
    delay_timer: ByteRegister,
    sound_timer: ByteRegister,
    ram: RAM,
    pub draw_flag: bool,
    pub keys_pressed: [bool; 16]
}

impl CPU{
    pub fn new(ram: RAM) -> CPU{
        //Regs from V0 to VF
        let mut v_regs:[ByteRegister; 16] =
            [ByteRegister::new(String::from("V0")),
            ByteRegister::new(String::from("V1")),
            ByteRegister::new(String::from("V2")),
            ByteRegister::new(String::from("V3")),
            ByteRegister::new(String::from("V4")),
            ByteRegister::new(String::from("V5")),
            ByteRegister::new(String::from("V6")),
            ByteRegister::new(String::from("V7")),
            ByteRegister::new(String::from("V8")),
            ByteRegister::new(String::from("V9")),
            ByteRegister::new(String::from("VA")),
            ByteRegister::new(String::from("VB")),
            ByteRegister::new(String::from("VC")),
            ByteRegister::new(String::from("VD")),
            ByteRegister::new(String::from("VE")),
            ByteRegister::new(String::from("VF"))];
        //stack regs
        let mut stack_regs:[WordRegister; 16] = [WordRegister::new(String::from("Stack0")),
            WordRegister::new(String::from("Stack1")),
            WordRegister::new(String::from("Stack2")),
            WordRegister::new(String::from("Stack3")),
            WordRegister::new(String::from("Stack4")),
            WordRegister::new(String::from("Stack5")),
            WordRegister::new(String::from("Stack6")),
            WordRegister::new(String::from("Stack7")),
            WordRegister::new(String::from("Stack8")),
            WordRegister::new(String::from("Stack9")),
            WordRegister::new(String::from("StackA")),
            WordRegister::new(String::from("StackB")),
            WordRegister::new(String::from("StackC")),
            WordRegister::new(String::from("StackD")),
            WordRegister::new(String::from("StackE")),
            WordRegister::new(String::from("StackF"))];


        let mut cpu = CPU{
            ram,
            v: v_regs,
            index: WordRegister::new(String::from("Index")),
            program_counter: WordRegister::new(String::from("Program Counter")),
            stack_pointer: ByteRegister::new(String::from("Stack Pointer")),
            stack: stack_regs,
            delay_timer: ByteRegister::new(String::from("Delay timer")),
            sound_timer: ByteRegister::new(String::from("Sound timer")),
            draw_flag: false,
            keys_pressed: [false; 16]
        };
        cpu.program_counter.write_reg(0x200);
        cpu
    }

    pub fn cycle(&mut self, gfx: &mut GraphicsBuffer){
        //fetch
        let curr_address = self.program_counter.read_reg();
        let instruction = self.ram.read_word(curr_address);
        self.program_counter.write_reg(curr_address + 2);

        //nibbles
        let op_1 = (instruction & 0xF000) >> 12;
        let op_2 = (instruction & 0x0F00) >> 8;
        let op_3 = (instruction & 0x00F0) >> 4;
        let op_4 = instruction & 0x000F;

        //commonly encountered expressions
        let x = op_2 as usize;
        let vx = self.v[x].read_reg();
        let y = op_3 as usize;
        let vy = self.v[y].read_reg();
        let n = op_4 as u8;
        let kk = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        //decode/exec
        match (op_1, op_2, op_3, op_4){
            (0, 0, 0, 0) => {}
            (0, 0, 0xE, 0) => {
                gfx.clear();
                self.draw_flag = true;
            }
            (0, 0, 0xE, 0xE) => {
                self.program_counter.write_reg(self.stack[self.stack_pointer.read_reg() as usize].read_reg());
                self.stack_pointer.decrement_reg();
            }
            (1, _, _, _) => {
                self.program_counter.write_reg(nnn);
            }
            (2, _, _, _) => {
                self.stack_pointer.increment_reg();
                self.stack[self.stack_pointer.read_reg() as usize].write_reg(self.program_counter.read_reg());
                self.program_counter.write_reg(nnn);
            }
            (3, _, _, _) => {
                if vx == kk{
                    self.program_counter.skip_instruction();
                }
            }
            (4, _, _, _) => {
                if vx != kk{
                    self.program_counter.skip_instruction();
                }
            }
            (5, _, _, 0) => {
                if vx == vy{
                    self.program_counter.skip_instruction();
                }
            }
            (6, _, _, _) => {
                self.v[x].write_reg(kk);
            }
            (7, _, _, _) => {
                self.v[x].write_reg(u8::wrapping_add(vx,  kk ));
            }
            (8, _, _, 0) => {
                self.v[x].write_reg(vy);
            }
            (8, _, _, 1) => {
                self.v[x].write_reg(vx | vy);
            }
            (8, _, _, 2) => {
                self.v[x].write_reg(vx & vy);
            }
            (8, _, _, 3) => {
                self.v[x].write_reg(vx ^ vy);
            }
            (8, _, _, 4) => {
                let sum = vx as u16 + vy as u16;
                if sum & 0x0100 != 0{
                    self.v[0xF].write_reg(1);
                }
                self.v[x].write_reg((sum & 0xFF) as u8);
            }
            (8, _, _, 5) => {
                if vx > vy{
                    self.v[0xF].write_reg(1);
                } else {
                    self.v[0xF].write_reg(0);
                }
                self.v[x].write_reg(u8::wrapping_sub(vx, vy));
            }
            (8, _, _, 6) => {
                if vx & 1 != 0 {
                    self.v[0xF].write_reg(1);
                } else {
                    self.v[0xF].write_reg(0);
                }
                self.v[x].write_reg(vx >> 1);
            }
            (8, _, _, 7) => {
                if vx < vy{
                    self.v[0xF].write_reg(1);
                } else {
                    self.v[0xF].write_reg(0);
                }
                self.v[x].write_reg(u8::wrapping_sub(vy, vx));
            }
            (8, _, _, 0xE) => {
                if vx & 0x80 != 0 {
                    self.v[0xF].write_reg(1);
                } else {
                    self.v[0xF].write_reg(0);
                }
                self.v[x].write_reg(vx << 1);
            }
            (9, _, _, 0) => {
                if vx != vy{
                    self.program_counter.skip_instruction();
                }
            }
            (0xA, _, _, _) => {
                self.index.write_reg(nnn);
            }
            (0xB, _, _, _) => {
                self.program_counter.write_reg(nnn + self.v[x].read_reg() as u16);
            }
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                let ran_u8: u8 = rng.gen();
                self.v[x].write_reg(ran_u8 & kk);
            }
            (0xD, _, _, _) => {
                let x_coord = vx & 0x3F;
                let y_coord = vy & 0x1F;
                self.v[0xF].write_reg(0);
                for row in 0..n{
                    //note that each row of the sprite is 8 pixels, and thus one byte long
                    let this_row = self.ram.read_byte(self.index.read_reg() + (row) as u16);
                    if y_coord + row <= 0x1F{
                        for column in 0..8{
                            if x_coord + column <= 0x3F && (this_row & (0x80 >> column)) != 0{
                                if gfx.get(x_coord + column, y_coord + row){
                                    self.v[0xF].write_reg(1);
                                }
                                gfx.toggle(x_coord + column, y_coord + row);
                            }
                        }
                    }
                }
                self.draw_flag = true;
            }
            (0xE, _, 9, 0xE) => {
                if self.keys_pressed[self.v[x].read_reg() as usize] {
                    self.program_counter.skip_instruction();
                };
            }
            (0xE, _, 0xA, 1) => {
                if !self.keys_pressed[self.v[x].read_reg() as usize] {
                    self.program_counter.skip_instruction();
                };
            }
            (0xF, _, 0, 7) => {
                self.v[x].write_reg(self.delay_timer.read_reg());
            }
            (0xF, _, 0, 0xA) => {
                self.program_counter.wait_instruction();
                for i in 0..0x10{
                    if self.keys_pressed[i]{
                        self.v[x].write_reg(i as u8);
                        self.program_counter.skip_instruction();
                        break;
                    }
                }
            }
            (0xF, _, 1, 5) => {
                self.delay_timer.write_reg(vx);
            }
            (0xF, _, 1, 8) => {
                self.sound_timer.write_reg(vx);
            }
            (0xF, _, 1, 0xE) => {
                self.index.write_reg(self.index.read_reg() + vx as u16);
            }
            (0xF, _, 2, 9) => {
                self.index.write_reg(0x50 + (5 * vx as u16));
            }
            (0xF, _, 3, 3) => {
                let location = self.index.read_reg();
                self.ram.write_byte(location, vx/100);
                self.ram.write_byte(location + 1, (vx/10)%10);
                self.ram.write_byte(location + 2, vx%10);
            }
            (0xF, _, 5, 5) => {
                let location = self.index.read_reg();
                for i  in 0..(x+1){
                    self.ram.write_byte(location + i as u16, self.v[i].read_reg());
                }
            }
            (0xF, _, 6, 5) => {
                let location = self.index.read_reg();
                for i  in 0..(x+1){
                    self.v[i].write_reg(self.ram.read_byte(location + i as u16));
                }
            }
            _ => {
               println!("Missing Opcode {:04X}", instruction);
            }
        }
    }
    pub fn timer(&mut self){
        if self.delay_timer.read_reg() > 0{
            self.delay_timer.decrement_reg();
        }
        if self.sound_timer.read_reg() > 0{
            self.sound_timer.decrement_reg();
        }
    }
}