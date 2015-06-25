use std::io::prelude::*;
use std::fs::File;
fn format(hex: u8) -> String {
    format!("{:01$x}", hex, 2)
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
            0x09 => { println!("DAD     B"); pc = pc + 1; },
            0x0a => { println!("LDAX    B"); pc = pc + 1; },
            0x0b => { println!("DCX     B"); pc = pc + 1; },
            0x0c => { println!("INR     C"); pc = pc + 1; },
            0x0d => { println!("DCR     C"); pc = pc + 1; },
            0x0e => { println!("MVI     C,${}", format(buffer[pc+1])); pc = pc + 2; },
            0x0f => { println!("RRC"); pc = pc + 1; },
            0x10 => { println!("NOP"); pc = pc + 1; },
            0x11 => { println!("LXI     D,${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x12 => { println!("STAX    D"); pc = pc + 1; },
            0x13 => { println!("INX     D"); pc = pc + 1; },
            0x14 => { println!("INR     D"); pc = pc + 1; },
            0x15 => { println!("DCR     D"); pc = pc + 1; },
            0x16 => { println!("MVI     D,${}", format(buffer[pc+1])); pc = pc + 2; },
            0x17 => { println!("RAL"); pc = pc + 1; },
            0x18 => { println!("NOP"); pc = pc + 1; },
            0x19 => { println!("DAD     D"); pc = pc + 1; },
            0x1a => { println!("LDAX    D"); pc = pc + 1; },
            0x1b => { println!("DCX     D"); pc = pc + 1; },
            0x1c => { println!("INR     E"); pc = pc + 1; },
            0x1d => { println!("DCR     E"); pc = pc + 1; },
            0x1e => { println!("MVI     E,${}", format(buffer[pc+1])); pc = pc + 2; },
            0x1f => { println!("RAR"); pc = pc + 1; },
            0x20 => { println!("RIM"); pc = pc + 1; },
            0x21 => { println!("LXI     H,${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x22 => { println!("SHLD    ${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x23 => { println!("INX     H"); pc = pc + 1; },
            0x24 => { println!("INR     H"); pc = pc + 1; },
            0x25 => { println!("DCR     H"); pc = pc + 1; },
            0x26 => { println!("MVI     H,${}", format(buffer[pc+1])); pc = pc + 2; },
            0x27 => { println!("DAA"); pc = pc + 1; },
            0x28 => { println!("NOP"); pc = pc + 1; },
            0x29 => { println!("DAD     H"); pc = pc + 1; },
            0x2a => { println!("LHLD    ${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x2b => { println!("DCX     H"); pc = pc + 1; },
            0x2c => { println!("INR     L"); pc = pc + 1; },
            0x2d => { println!("DCR     L"); pc = pc + 1; },
            0x2e => { println!("MVI     L,${}", format(buffer[pc+1])); pc = pc + 2; },
            0x2f => { println!("CMA"); pc = pc + 1; },
            0x30 => { println!("SIM"); pc = pc + 1; },
            0x31 => { println!("LXI     SP,${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x32 => { println!("STA     ${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x33 => { println!("INX     SP"); pc = pc + 1; },
            0x34 => { println!("INR     M"); pc = pc + 1; },
            0x35 => { println!("DCR     M"); pc = pc + 1; },
            0x36 => { println!("MVI     M,${}", format(buffer[pc+1])); pc = pc + 2; },
            0x37 => { println!("STC"); pc = pc + 1; },
            0x38 => { println!("NOP"); pc = pc + 1; },


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
