extern crate sdl2;
use crate::ram::RAM;
use crate::cpu::CPU;
use crate::graphics_buffer::GraphicsBuffer;
use crate::speaker::Speaker;

use std::thread::sleep;
use std::time::{Duration, SystemTime};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env::args;

mod ram;
mod cpu;
mod byte_register;
mod word_register;
mod graphics_buffer;
mod speaker;

const PIXEL_WIDTH: u32 = 16;
const HEIGHT: u32 = 32*PIXEL_WIDTH;
const WIDTH: u32 = 64*PIXEL_WIDTH;

fn main() {
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    // backend code
    let mut ram = RAM::new();
    ram.load_rom(String::from(filename));
    let mut cpu = CPU::new(ram);
    let mut gfx = GraphicsBuffer::new();

    //sdl2 code
    let sdl = sdl2::init().expect("Could not initalize sdl");
    let video_subsystem = sdl.video().expect("Could not initalize video subsystem");
    let window = video_subsystem.window("Chip-8", WIDTH, HEIGHT)
    .position_centered().build().expect("Could not initialize window");
    let mut canvas = window.into_canvas().build().expect("Could not create canvas");
    let mut event_pump = sdl.event_pump().expect("Could not initliaze event handler");
    let mut last_tick_time = SystemTime::now();
    let mut speaker = Speaker::new(sdl);

    //main loop
    loop {
        //input
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
        //cpu
        cpu.cycle(&mut gfx);
        //render
        if cpu.draw_flag{
            gfx.render(&mut canvas);
            cpu.draw_flag = false;
        }
        //audio
        if cpu.sound_flag{
            speaker.start();
        } else {
            speaker.stop();
        }
        //timer
        if last_tick_time.elapsed().expect("Clock error") >= Duration::new(0, 1_000_000_000u32 / 60){
            cpu.timer();
            last_tick_time = SystemTime::now()
        }
        //sleep(Duration::from_millis(1));
        sleep(Duration::new(0, 1_000_000_000u32 / 2500))
     }

}
