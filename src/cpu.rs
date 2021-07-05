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
    stack: [WordRegister; 17],
    stack_pointer: ByteRegister,
    sound_timer: ByteRegister,
    delay_timer: ByteRegister,
    ram: RAM,
    pub draw_flag: bool,
    pub sound_flag: bool,
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
        let mut stack_regs:[WordRegister; 17] = [WordRegister::new(String::from("dummy")),
            WordRegister::new(String::from("Stack0")),
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
            sound_flag: false,
            keys_pressed: [false; 16]
        };
        cpu.program_counter.write_reg(0x200);
        cpu
    }

    pub fn cycle(&mut self, gfx: &mut GraphicsBuffer){
        //fetch
        let curr_address = self.program_counter.read_reg();
        let instruction = self.ram.read_word(curr_address);
        self.program_counter.next_instruction();

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
                    self.program_counter.next_instruction();
                }
            }
            (4, _, _, _) => {
                if vx != kk{
                    self.program_counter.next_instruction();
                }
            }
            (5, _, _, 0) => {
                if vx == vy{
                    self.program_counter.next_instruction();
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
                if vx >= vy{
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
                if vx <= vy{
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
                    self.program_counter.next_instruction();
                }
            }
            (0xA, _, _, _) => {
                self.index.write_reg(nnn);
            }
            (0xB, _, _, _) => {
                self.program_counter.write_reg(nnn + self.v[0].read_reg() as u16);
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
                    self.program_counter.next_instruction();
                };
            }
            (0xE, _, 0xA, 1) => {
                if !self.keys_pressed[self.v[x].read_reg() as usize] {
                    self.program_counter.next_instruction();
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
                        self.program_counter.next_instruction();
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
            self.sound_flag = true;
            self.sound_timer.decrement_reg();
        } else {
            self.sound_flag = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_test() {
        let mut ram = RAM::new();
        let mut cpu = CPU::new(ram);
        cpu.sound_timer.write_reg(2);
        cpu.delay_timer.write_reg(2);
        cpu.timer();
        assert_eq!(1, cpu.sound_timer.read_reg());
        assert_eq!(1, cpu.delay_timer.read_reg());
        cpu.timer();
        assert_eq!(0, cpu.sound_timer.read_reg());
        assert_eq!(0, cpu.delay_timer.read_reg());
        cpu.timer();
        assert_eq!(0, cpu.sound_timer.read_reg());
        assert_eq!(0, cpu.delay_timer.read_reg());
    }

    #[test]
    fn cls_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x00E0);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        gfx.toggle(0,0);
        gfx.toggle(0xF,0xF);
        cpu.cycle(&mut gfx);
        assert_eq!(false, gfx.get(0,0));
        assert_eq!(false, gfx.get(0xF,0xF));
        assert_eq!(true, cpu.draw_flag);
    }

    fn ret_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x00EE);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.stack_pointer.write_reg(1);
        cpu.stack[1].write_reg(0xABC);
        cpu.cycle(&mut gfx);
        assert_eq!(0xABC, cpu.program_counter.read_reg());
        assert_eq!(0, cpu.stack_pointer.read_reg());
    }

    #[test]
    fn jp_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x1FFF);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0xFFF, cpu.program_counter.read_reg());
    }

    #[test]
    fn call_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x2069);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x202, cpu.stack[1].read_reg());
        assert_eq!(0x069, cpu.program_counter.read_reg());
        assert_eq!(1, cpu.stack_pointer.read_reg());
    }

    #[test]
    fn se_imm_eq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x3781);
        let mut cpu = CPU::new(ram);
        cpu.v[7].write_reg(0x81);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x204, cpu.program_counter.read_reg());
    }

    #[test]
    fn se_imm_neq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x3781);
        let mut cpu = CPU::new(ram);
        cpu.v[7].write_reg(0x80);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x202, cpu.program_counter.read_reg());
    }

    #[test]
    fn sne_eq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x4781);
        let mut cpu = CPU::new(ram);
        cpu.v[7].write_reg(0x81);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x202, cpu.program_counter.read_reg());
    }

    #[test]
    fn sne_neq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x4781);
        let mut cpu = CPU::new(ram);
        cpu.v[7].write_reg(0x80);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x204, cpu.program_counter.read_reg());
    }

    #[test]
    fn se_mem_eq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x55C0);
        let mut cpu = CPU::new(ram);
        cpu.v[5].write_reg(0xA2);
        cpu.v[0xC].write_reg(0xA2);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x204, cpu.program_counter.read_reg());
    }

    #[test]
    fn se_mem_neq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x55C0);
        let mut cpu = CPU::new(ram);
        cpu.v[5].write_reg(0xA2);
        cpu.v[0xC].write_reg(0xA3);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x202, cpu.program_counter.read_reg());
    }

    #[test]
    fn ld_imm_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x6A22);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x22, cpu.v[0xA].read_reg());
    }

    #[test]
    fn add_imm_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x7756);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[7].write_reg(0x37);
        cpu.cycle(&mut gfx);
        assert_eq!(0x8D, cpu.v[7].read_reg());
    }

    #[test]
    fn add_imm_overflow_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x77EE);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[7].write_reg(0xEE);
        cpu.cycle(&mut gfx);
        assert_eq!(0xDC, cpu.v[7].read_reg());
    }

    #[test]
    fn ld_mem_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8E20);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[2].write_reg(0xD1);
        cpu.cycle(&mut gfx);
        assert_eq!(0xD1, cpu.v[0xE].read_reg());
    }

    #[test]
    fn or_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8CE1);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xC].write_reg(0x33);
        cpu.v[0xE].write_reg(0x55);
        cpu.cycle(&mut gfx);
        assert_eq!(0x77, cpu.v[0xC].read_reg());
    }

    #[test]
    fn and_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8CE2);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xC].write_reg(0x33);
        cpu.v[0xE].write_reg(0x55);
        cpu.cycle(&mut gfx);
        assert_eq!(0x11, cpu.v[0xC].read_reg());
    }

    #[test]
    fn xor_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8CE3);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xC].write_reg(0x33);
        cpu.v[0xE].write_reg(0x55);
        cpu.cycle(&mut gfx);
        assert_eq!(0x66, cpu.v[0xC].read_reg());
    }

    #[test]
    fn add_mem_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8BC4);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0x45);
        cpu.v[0xC].write_reg(0x67);
        cpu.cycle(&mut gfx);
        assert_eq!(0xAC, cpu.v[0xB].read_reg());
        assert_eq!(0, cpu.v[0xF].read_reg());
    }

    #[test]
    fn add_mem_overflow_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8BC4);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0xEC);
        cpu.v[0xC].write_reg(0x34);
        cpu.cycle(&mut gfx);
        assert_eq!(0x20, cpu.v[0xB].read_reg());
        assert_eq!(1, cpu.v[0xF].read_reg());
    }

    #[test]
    fn sub_no_borrow_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8BC5);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0x3A);
        cpu.v[0xC].write_reg(0x24);
        cpu.cycle(&mut gfx);
        assert_eq!(0x16, cpu.v[0xB].read_reg());
        assert_eq!(1, cpu.v[0xF].read_reg());
    }

    #[test]
    fn sub_borrow_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8BC5);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0x5C);
        cpu.v[0xC].write_reg(0xA4);
        cpu.cycle(&mut gfx);
        assert_eq!(0xB8, cpu.v[0xB].read_reg());
        assert_eq!(0, cpu.v[0xF].read_reg());
    }

    #[test]
    fn shr_one_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8B06);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0x1B);
        cpu.cycle(&mut gfx);
        assert_eq!(0xD, cpu.v[0xB].read_reg());
        assert_eq!(1, cpu.v[0xF].read_reg());
    }

    #[test]
    fn shr_zero_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8B06);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0x1C);
        cpu.cycle(&mut gfx);
        assert_eq!(0xE, cpu.v[0xB].read_reg());
        assert_eq!(0, cpu.v[0xF].read_reg());
    }

    #[test]
    fn subn_no_borrow_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8BC7);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0x24);
        cpu.v[0xC].write_reg(0x3A);
        cpu.cycle(&mut gfx);
        assert_eq!(0x16, cpu.v[0xB].read_reg());
        assert_eq!(1, cpu.v[0xF].read_reg());
    }

    #[test]
    fn subn_borrow_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8BC7);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0xA4);
        cpu.v[0xC].write_reg(0x5C);
        cpu.cycle(&mut gfx);
        assert_eq!(0xB8, cpu.v[0xB].read_reg());
        assert_eq!(0, cpu.v[0xF].read_reg());
    }

    #[test]
    fn shl_one_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8B0E);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0xE5);
        cpu.cycle(&mut gfx);
        assert_eq!(0xCA, cpu.v[0xB].read_reg());
        assert_eq!(1, cpu.v[0xF].read_reg());
    }

    #[test]
    fn shl_zero_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x8B0E);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0xB].write_reg(0x5E);
        cpu.cycle(&mut gfx);
        assert_eq!(0xBC, cpu.v[0xB].read_reg());
        assert_eq!(0, cpu.v[0xF].read_reg());
    }

    #[test]
    fn sne_mem_eq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x95C0);
        let mut cpu = CPU::new(ram);
        cpu.v[5].write_reg(0xA2);
        cpu.v[0xC].write_reg(0xA2);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x202, cpu.program_counter.read_reg());
    }

    #[test]
    fn sne_mem_neq_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0x95C0);
        let mut cpu = CPU::new(ram);
        cpu.v[5].write_reg(0xA2);
        cpu.v[0xC].write_reg(0xA3);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0x204, cpu.program_counter.read_reg());
    }

    #[test]
    fn ld_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xACCC);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.cycle(&mut gfx);
        assert_eq!(0xCCC, cpu.index.read_reg());
    }

    #[test]
    fn jp_mem_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xBCCC);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[0].write_reg(0x11);
        cpu.cycle(&mut gfx);
        assert_eq!(0xCDD, cpu.program_counter.read_reg());
    }
    //TODO: DRW test

    #[test]
    fn skp_pressed_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xE39E);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xA);
        cpu.keys_pressed[0xA] = true;
        cpu.cycle(&mut gfx);
        assert_eq!(0x204, cpu.program_counter.read_reg());
    }

    #[test]
    fn skp_not_pressed_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xE39E);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xA);
        cpu.cycle(&mut gfx);
        assert_eq!(0x202, cpu.program_counter.read_reg());
    }

    #[test]
    fn sknp_pressed_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xE3A1);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xA);
        cpu.keys_pressed[0xA] = true;
        cpu.cycle(&mut gfx);
        assert_eq!(0x202, cpu.program_counter.read_reg());
    }

    #[test]
    fn sknp_not_pressed_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xE3A1);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xA);
        cpu.cycle(&mut gfx);
        assert_eq!(0x204, cpu.program_counter.read_reg());
    }


    #[test]
    fn ld_from_dt_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xF307);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.delay_timer.write_reg(0xCC);
        cpu.cycle(&mut gfx);
        assert_eq!(0xCC, cpu.v[3].read_reg());
    }
    //TODO: LD key test

    #[test]
    fn ld_into_dt_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xF315);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xCC);
        cpu.cycle(&mut gfx);
        assert_eq!(0xCC, cpu.delay_timer.read_reg());
    }

    #[test]
    fn ld_into_st_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xF318);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xCC);
        cpu.cycle(&mut gfx);
        assert_eq!(0xCC, cpu.sound_timer.read_reg());
    }

    #[test]
    fn add_index_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xF31E);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.index.write_reg(0x2211);
        cpu.v[3].write_reg(0xCC);
        cpu.cycle(&mut gfx);
        assert_eq!(0x22DD, cpu.index.read_reg());
    }

    #[test]
    fn ld_sprite_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xF329);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xA);
        cpu.cycle(&mut gfx);
        assert_eq!(0x82, cpu.index.read_reg());
    }

    #[test]
    fn bin_dec_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xF333);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xA2);
        cpu.index.write_reg(0x400);
        cpu.cycle(&mut gfx);
        assert_eq!(1, cpu.ram.read_byte(0x400));
        assert_eq!(6, cpu.ram.read_byte(0x401));
        assert_eq!(2, cpu.ram.read_byte(0x402));
    }

    #[test]
    fn ld_from_reg_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xFF55);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.v[3].write_reg(0xA2);
        cpu.v[7].write_reg(0x13);
        cpu.index.write_reg(0x400);
        cpu.cycle(&mut gfx);
        assert_eq!(0xA2, cpu.ram.read_byte(0x403));
        assert_eq!(0x13, cpu.ram.read_byte(0x407));
    }

    #[test]
    fn ld_to_reg_test() {
        let mut ram = RAM::new();
        ram.write_word(0x200, 0xFF65);
        ram.write_byte(0x403, 0xA2);
        ram.write_byte(0x407, 0x13);
        let mut cpu = CPU::new(ram);
        let mut gfx = GraphicsBuffer::new();
        cpu.index.write_reg(0x400);
        cpu.cycle(&mut gfx);
        assert_eq!(0xA2, cpu.v[3].read_reg());
        assert_eq!(0x13, cpu.v[7].read_reg());
    }
}