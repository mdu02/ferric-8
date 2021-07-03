extern crate sdl2;
use crate::ram::RAM;
use crate::cpu::CPU;
use crate::graphics_buffer::GraphicsBuffer;
use std::thread::sleep;
use std::time::Duration;
use std::fs::read;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

mod ram;
mod cpu;
mod byte_register;
mod word_register;
mod graphics_buffer;

const PIXEL_WIDTH: u32 = 16;
const HEIGHT: u32 = 32*PIXEL_WIDTH;
const WIDTH: u32 = 64*PIXEL_WIDTH;

fn main() {
    // backend code
    let mut ram = RAM::new();
    load_rom(&mut ram, String::from("roms/Pong.ch8"));
    let mut cpu = CPU::new(ram);
    let mut gfx = GraphicsBuffer::new();

    //sdl2 code
    let sdl = sdl2::init().expect("Could not initalize sdl");
    let video_subsystem = sdl.video().expect("Could not initalize video subsystem");
    let window = video_subsystem.window("Chip-8", WIDTH, HEIGHT)
    .position_centered().build().expect("Could not initialize window");
    let mut canvas = window.into_canvas().build().expect("Could not create canvas");
    let mut event_pump = sdl.event_pump().expect("Could not initliaze event handler");

    //main loop
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { cpu.keys_pressed[1] = true },
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { cpu.keys_pressed[2] = true },
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { cpu.keys_pressed[3] = true },
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { cpu.keys_pressed[0xC] = true },
                Event::KeyDown { keycode: Some(Keycode::Q), .. } => { cpu.keys_pressed[4] = true },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { cpu.keys_pressed[5] = true },
                Event::KeyDown { keycode: Some(Keycode::E), .. } => { cpu.keys_pressed[6] = true },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => { cpu.keys_pressed[0xD] = true },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { cpu.keys_pressed[7] = true },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { cpu.keys_pressed[8] = true },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { cpu.keys_pressed[9] = true },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => { cpu.keys_pressed[0xE] = true },
                Event::KeyDown { keycode: Some(Keycode::Z), .. } => { cpu.keys_pressed[0xA] = true },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => { cpu.keys_pressed[0] = true },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => { cpu.keys_pressed[0xB] = true },
                Event::KeyDown { keycode: Some(Keycode::V), .. } => { cpu.keys_pressed[0xF] = true },
                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => { cpu.keys_pressed[1] = false },
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => { cpu.keys_pressed[2] = false },
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => { cpu.keys_pressed[3] = false },
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => { cpu.keys_pressed[0xC] = false },
                Event::KeyUp { keycode: Some(Keycode::Q), .. } => { cpu.keys_pressed[4] = false },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => { cpu.keys_pressed[5] = false },
                Event::KeyUp { keycode: Some(Keycode::E), .. } => { cpu.keys_pressed[6] = false },
                Event::KeyUp { keycode: Some(Keycode::R), .. } => { cpu.keys_pressed[0xD] = false },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => { cpu.keys_pressed[7] = false },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => { cpu.keys_pressed[8] = false },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => { cpu.keys_pressed[9] = false },
                Event::KeyUp { keycode: Some(Keycode::F), .. } => { cpu.keys_pressed[0xE] = false },
                Event::KeyUp { keycode: Some(Keycode::Z), .. } => { cpu.keys_pressed[0xA] = false },
                Event::KeyUp { keycode: Some(Keycode::X), .. } => { cpu.keys_pressed[0] = false },
                Event::KeyUp { keycode: Some(Keycode::C), .. } => { cpu.keys_pressed[0xB] = false },
                Event::KeyUp { keycode: Some(Keycode::V), .. } => { cpu.keys_pressed[0xF] = false },
                _ => {}
            }
        }
        cpu.cycle(&mut gfx);
        if cpu.draw_flag{
         render(&mut canvas, &gfx);
         cpu.draw_flag = false;
        }
        cpu.timer();
        //sleep(Duration::from_millis(1));
        sleep(Duration::new(0, 1_000_000_000u32 / 2500))
     }

}

fn render(canvas: &mut WindowCanvas, gfx: &GraphicsBuffer){
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(Color::WHITE);
    for i in 0..64{
        for j in 0..32{
            if gfx.get(i, j){
                canvas.fill_rect(
                    Rect::new((PIXEL_WIDTH * i as u32) as i32, (PIXEL_WIDTH * j as u32) as i32,
                              PIXEL_WIDTH, PIXEL_WIDTH)
                );
            }
        }
    }
    canvas.present();
}

fn load_rom(ram: &mut RAM, filename: String){
    let rom_bytes = read(filename).unwrap();
    let mut address: u16 = 0x200;
    for i in &rom_bytes{
        ram.write_byte(address, *i);
        address += 1;
    }
}