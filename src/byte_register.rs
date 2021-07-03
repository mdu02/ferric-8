pub struct ByteRegister {
    value: u8,
    name: String
}

impl ByteRegister {
    pub fn new(name: String) -> ByteRegister {
        let mut reg : ByteRegister = ByteRegister { value: 0, name};
        reg
    }

    pub fn read_reg(&self) -> u8 {
        self.value
    }

    pub fn write_reg(&mut self, val: u8) {
        self.value = val
    }

    pub fn increment_reg(&mut self) {
        self.value += 1;
    }

    pub fn decrement_reg(&mut self) {
        self.value -= 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write(){
        let mut br = ByteRegister::new(String::new());
        br.write_reg(0x76);
        assert_eq!(0x76, br.read_reg());
    }

    #[test]
    fn increment(){
        let mut br = ByteRegister::new(String::new());
        br.write_reg(0x50);
        br.increment_reg();
        assert_eq!(0x51, br.read_reg());
    }

    #[test]
    fn decrement(){
        let mut br = ByteRegister::new(String::new());
        br.write_reg(0x50);
        br.decrement_reg();
        assert_eq!(0x4F, br.read_reg());
    }
}
