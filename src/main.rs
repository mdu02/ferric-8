use crate::ram::Ram;

mod ram;
mod cpu;

fn main() {
    let ram = Ram::new();
    ram.core_dump();
}