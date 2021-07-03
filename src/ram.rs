pub struct RAM {
    memory: [u8; 4096]
}

impl RAM {
    pub fn new() -> RAM {
        let mut ram = RAM {memory: [0; 4096]};
        let font :[u8; 80] =
            [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
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
            0xF0, 0x80, 0xF0, 0x80, 0x80]; // F

        for i in 0..80 {
            ram.write_byte(i + 0x50, font[i as usize])
        }
        ram
    }

    pub fn read_byte(&self, adr: u16) -> u8 {
        self.memory[adr as usize]
    }

    pub fn write_byte(&mut self, adr: u16, val: u8) {
        self.memory[adr as usize] = val
    }

    pub fn read_word(&self, adr: u16) -> u16 {
        let first_byte = self.memory[adr as usize] as u16;
        let second_byte =  self.memory[(adr + 1) as usize] as u16;
        (first_byte << 8) + second_byte
    }

    pub fn write_word(&mut self, adr: u16, val: u16) {
        let first_byte = (val >> 8) as u8;
        let second_byte =  (val & (0x00FF)) as u8;
        self.memory[adr as usize] = first_byte;
        self.memory[(adr + 1) as usize] = second_byte;
    }

    pub fn core_dump(&self) {
        print!("Offset");
        for i in 0..16{
            print!(" {:02X}", i);
        }
        println!();
        for i in 0..256{
            print!("0x{:04X}", i*16);
            for j in 0..16{
                print!(" {:02X}", self.read_byte(i*16 + j))
            }
            println!();
        }

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write_byte() {
        let mut ram = RAM::new();
        ram.write_byte(0x1D5, 0xAC);
        assert_eq!(0xAC, ram.read_byte(0x1D5));
    }

    #[test]
    fn read_write_word() {
        let mut ram = RAM::new();
        ram.write_word(0x1D2, 0xABCD);
        assert_eq!(0xABCD, ram.read_word(0x1D2));
    }

    #[test]
    fn write_word_read_bytes() {
        let mut ram = RAM::new();
        ram.write_word(0x1D2, 0xABCD);
        assert_eq!(0xAB, ram.read_byte(0x1D2));
        assert_eq!(0xCD, ram.read_byte(0x1D3));
    }

    #[test]
    fn write_bytes_read_word() {
        let mut ram = RAM::new();
        ram.write_byte(0x3A2, 0x81);
        ram.write_byte(0x3A3, 0x2C);
        assert_eq!(0x812C, ram.read_word(0x3A2));
    }
}
