use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;

const PIXEL_WIDTH: u32 = 16;
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct GraphicsBuffer{
    pixels: [[bool; WIDTH];HEIGHT]
}

impl GraphicsBuffer{
    pub fn new() -> GraphicsBuffer{
        let gfx = GraphicsBuffer{
            pixels: [[false; WIDTH];HEIGHT]
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
        for i in 0..HEIGHT{
            for j in 0..WIDTH{
                self.pixels[i][j] = false;
            }
        }
    }

    pub fn draw_to_console(&self){
        for i in 0..HEIGHT{
            for j in 0..WIDTH{
                print!("{}", if self.pixels[i][j] {'█'} else {'░'});
            }
            println!();
        }
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas){
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);
        for i in 0..WIDTH{
            for j in 0..HEIGHT{
                if self.get(i as u8, j as u8){
                    canvas.fill_rect(
                        Rect::new((PIXEL_WIDTH * i as u32) as i32, (PIXEL_WIDTH * j as u32) as i32,
                                  PIXEL_WIDTH, PIXEL_WIDTH)
                    );
                }
            }
        }
        canvas.present();
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