use crate::ram::Ram;

pub struct Cpu {
    vx: [u8; 16],
    index: u8,
    pc: u8,
    stack: [u16; 16],
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
    ram: Ram
}

// impl Cpu{
//     pub
//
// }