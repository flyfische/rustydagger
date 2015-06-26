use std::fs::File;
use std::io::prelude::*;
mod disassemble;

fn main() {
    let mut rom = File::open("invaders.rom").unwrap();
    let mut buffer = vec![0; 256];
    let block = rom.read_to_end(&mut buffer);
    disassemble::disassemble(buffer);
}
