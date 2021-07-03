pub struct GraphicsBuffer{
    pixels: [[bool; 64];32]
}

impl GraphicsBuffer{
    pub fn new() -> GraphicsBuffer{
        let gfx = GraphicsBuffer{
            pixels: [[false; 64];32]
        };
        gfx
    }

    pub fn get(&self, x: u8, y: u8) -> bool{
        self.pixels[y as usize][x as usize]
    }

    pub fn toggle(&mut self, x: u8, y: u8){
        self.pixels[y as usize][x as usize] = !self.pixels[y as usize][x as usize];
    }

    pub fn clear(&mut self){
        for i in 0..32{
            for j in 0..64{
                self.pixels[i][j] = false;
            }
        }
    }

    pub fn draw_to_console(&self){
        for i in 0..32{
            for j in 0..64{
                print!("{}", if self.pixels[i][j] {'█'} else {'░'});
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let mut gfx = GraphicsBuffer::new();
        assert_eq!(false, gfx.get(0,0));
    }

    #[test]
    fn toggle_test() {
        let mut gfx = GraphicsBuffer::new();
        gfx.toggle(0,0);
        assert_eq!(true, gfx.get(0,0));
        gfx.toggle(0,0);
        assert_eq!(false, gfx.get(0,0));
    }

    #[test]
    fn clear_test() {
        let mut gfx = GraphicsBuffer::new();
        gfx.toggle(0,0);
        gfx.toggle(0xF,0xF);
        gfx.clear();
        assert_eq!(false, gfx.get(0,0));
        assert_eq!(false, gfx.get(0xF,0xF));
    }

}