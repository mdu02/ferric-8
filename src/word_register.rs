pub struct WordRegister {
    value: u16,
    name: String
}

impl WordRegister {
    pub fn new(name: String) -> WordRegister {
        let mut reg : WordRegister = WordRegister { value: 0, name};
        reg
    }

    pub fn read_reg(&self) -> u16 {
        self.value
    }

    pub fn write_reg(&mut self, val: u16) {
        self.value = val
    }

    pub fn increment_reg(&mut self) {
        self.value += 1;
    }

    pub fn decrement_reg(&mut self) {
        self.value -= 1;
    }

    pub fn next_instruction(&mut self) {
        self.value += 2;
    }

    pub fn wait_instruction(&mut self) {
        self.value -= 2;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_write(){
        let mut wr = WordRegister::new(String::new());
        wr.write_reg(0x2AE6);
        assert_eq!(0x2AE6, wr.read_reg());
    }

    #[test]
    fn increment_test(){
        let mut wr = WordRegister::new(String::new());
        wr.write_reg(0x4FFF);
        wr.increment_reg();
        assert_eq!(0x5000, wr.read_reg());
    }

    #[test]
    fn decrement_test(){
        let mut wr = WordRegister::new(String::new());
        wr.write_reg(0x7F00);
        wr.decrement_reg();
        assert_eq!(0x7EFF, wr.read_reg());
    }

    #[test]
    fn skip_test(){
        let mut wr = WordRegister::new(String::new());
        wr.write_reg(0x3FFE);
        wr.next_instruction();
        assert_eq!(0x4000, wr.read_reg());
    }

    #[test]
    fn wait_test(){
        let mut wr = WordRegister::new(String::new());
        wr.write_reg(0x5001);
        wr.wait_instruction();
        assert_eq!(0x4FFF, wr.read_reg());
    }
}
