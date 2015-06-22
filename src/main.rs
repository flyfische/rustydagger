use std::io::prelude::*;
use std::fs::File;
fn format(hex: u8) -> String {
    format!("{:x}", hex)
}
fn disassemble(buffer: Vec<u8>) {
    let mut pc: usize = 0;
    while pc < buffer.len() {
        match buffer[pc] {
            0x00 => { println!("NOP"); pc = pc + 1; },
            0x01 => { println!("LXI     B,${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x02 => { println!("STAX    B"); pc = pc + 1; },
            0x03 => { println!("INX     B"); pc = pc + 1; },
            0x04 => { println!("INR     B"); pc = pc + 1; },
            0x05 => { println!("DCR     B"); pc = pc + 1; },
            0x06 => { println!("MVI     B,${}", format(buffer[pc+1])); pc = pc + 2},
            0x07 => { println!("RLC"); pc = pc + 1; },
            0x08 => { println!("NOP"); pc = pc + 1; },
            _ => { println!("unknown"); pc = pc + 1; },
        }

    }
}

fn main() {
    let mut f = File::open("invaders.rom").unwrap();
    let mut buffer = vec![0; 256];
    let block = f.read_to_end(&mut buffer); 
    disassemble(buffer);
}
