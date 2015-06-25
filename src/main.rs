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
            0x39 => { println!("DAD     SP"); pc = pc + 1; },
            0x3a => { println!("LDA     ${}{}", format(buffer[pc+2]), format(buffer[pc+1])); pc = pc + 3; },
            0x3b => { println!("DCX     SP"); pc = pc + 1; },
            0x3c => { println!("INR     A"); pc = pc + 1; },
            0x3d => { println!("DCR     A"); pc = pc + 1; },
            0x3e => { println!("MVI     A,${}", format(buffer[pc+1])); pc = pc + 2; },
            0x3f => { println!("CMC"); pc = pc + 1; },
            0x40 => { println!("MOV     B,B"); pc = pc + 1; },
            0x41 => { println!("MOV     B,C"); pc = pc + 1; },
            0x42 => { println!("MOV     B,D"); pc = pc + 1; },
            0x43 => { println!("MOV     B,E"); pc = pc + 1; },
            0x44 => { println!("MOV     B,H"); pc = pc + 1; },
            0x45 => { println!("MOV     B,L"); pc = pc + 1; },
            0x46 => { println!("MOV     B,M"); pc = pc + 1; },
            0x47 => { println!("MOV     B,A"); pc = pc + 1; },
            0x48 => { println!("MOV     C,B"); pc = pc + 1; },
            0x49 => { println!("MOV     C,C"); pc = pc + 1; },
            0x4a => { println!("MOV     C,D"); pc = pc + 1; },
            0x4b => { println!("MOV     C,E"); pc = pc + 1; },
            0x4c => { println!("MOV     C,H"); pc = pc + 1; },
            0x4d => { println!("MOV     C,L"); pc = pc + 1; },
            0x4e => { println!("MOV     C,M"); pc = pc + 1; },
            0x4f => { println!("MOV     C,A"); pc = pc + 1; },
            0x50 => { println!("MOV     D,B"); pc = pc + 1; },
            0x51 => { println!("MOV     D,C"); pc = pc + 1; },
            0x52 => { println!("MOV     D,D"); pc = pc + 1; },
            0x53 => { println!("MOV     D,E"); pc = pc + 1; },
            0x54 => { println!("MOV     D,H"); pc = pc + 1; },
            0x55 => { println!("MOV     D,L"); pc = pc + 1; },
            0x56 => { println!("MOV     D,M"); pc = pc + 1; },
            0x57 => { println!("MOV     D,A"); pc = pc + 1; },
            0x58 => { println!("MOV     E,B"); pc = pc + 1; },
            0x59 => { println!("MOV     E,C"); pc = pc + 1; },
            0x5a => { println!("MOV     E,D"); pc = pc + 1; },
            0x5b => { println!("MOV     E,E"); pc = pc + 1; },
            0x5c => { println!("MOV     E,H"); pc = pc + 1; },
            0x5d => { println!("MOV     E,L"); pc = pc + 1; },
            0x5e => { println!("MOV     E,M"); pc = pc + 1; },
            0x5f => { println!("MOV     E,A"); pc = pc + 1; },
            0x60 => { println!("MOV     H,B"); pc = pc + 1; },
            0x61 => { println!("MOV     H,C"); pc = pc + 1; },
            0x62 => { println!("MOV     H,D"); pc = pc + 1; },
            0x63 => { println!("MOV     H,E"); pc = pc + 1; },
            0x64 => { println!("MOV     H,H"); pc = pc + 1; },
            0x65 => { println!("MOV     H,L"); pc = pc + 1; },
            0x66 => { println!("MOV     H,M"); pc = pc + 1; },
            0x67 => { println!("MOV     H,A"); pc = pc + 1; },
            0x68 => { println!("MOV     L,B"); pc = pc + 1; },
            0x69 => { println!("MOV     L,C"); pc = pc + 1; },
            0x6a => { println!("MOV     L,D"); pc = pc + 1; },
            0x6b => { println!("MOV     L,E"); pc = pc + 1; },
            0x6c => { println!("MOV     L,H"); pc = pc + 1; },
            0x6d => { println!("MOV     L,L"); pc = pc + 1; },
            0x6e => { println!("MOV     L,M"); pc = pc + 1; },
            0x6f => { println!("MOV     L,A"); pc = pc + 1; },
            0x70 => { println!("MOV     M,B"); pc = pc + 1; },
            0x71 => { println!("MOV     M,C"); pc = pc + 1; },
            0x72 => { println!("MOV     M,D"); pc = pc + 1; },
            0x73 => { println!("MOV     M,E"); pc = pc + 1; },
            0x74 => { println!("MOV     M,H"); pc = pc + 1; },
            0x75 => { println!("MOV     M,L"); pc = pc + 1; },
            0x76 => { println!("HLT"); pc = pc + 1; },
            0x77 => { println!("MOV     M,A"); pc = pc + 1; },
            0x78 => { println!("MOV     A,B"); pc = pc + 1; },
            0x79 => { println!("MOV     A,C"); pc = pc + 1; },
            0x7a => { println!("MOV     A,D"); pc = pc + 1; },
            0x7b => { println!("MOV     A,E"); pc = pc + 1; },
            0x7c => { println!("MOV     A,H"); pc = pc + 1; },
            0x7d => { println!("MOV     A,L"); pc = pc + 1; },
            0x7e => { println!("MOV     A,M"); pc = pc + 1; },
            0x7f => { println!("MOV     A,A"); pc = pc + 1; },
            0x80 => { println!("ADD     B"); pc = pc + 1; },
            0x81 => { println!("ADD     C"); pc = pc + 1; },
            0x82 => { println!("ADD     D"); pc = pc + 1; },
            0x83 => { println!("ADD     E"); pc = pc + 1; },
            0x84 => { println!("ADD     H"); pc = pc + 1; },
            0x85 => { println!("ADD     L"); pc = pc + 1; },
            0x86 => { println!("ADD     M"); pc = pc + 1; },
            0x87 => { println!("ADD     A"); pc = pc + 1; },
            0x88 => { println!("ADC     B"); pc = pc + 1; },
            0x89 => { println!("ADC     C"); pc = pc + 1; },
            0x8a => { println!("ADC     D"); pc = pc + 1; },
            0x8b => { println!("ADC     E"); pc = pc + 1; },
            0x8c => { println!("ADC     H"); pc = pc + 1; },
            0x8d => { println!("ADC     L"); pc = pc + 1; },
            0x8e => { println!("ADC     M"); pc = pc + 1; },
            0x8f => { println!("ADC     A"); pc = pc + 1; },
            0x90 => { println!("SUB     B"); pc = pc + 1; },
            0x91 => { println!("SUB     C"); pc = pc + 1; },
            0x92 => { println!("SUB     D"); pc = pc + 1; },
            0x93 => { println!("SUB     E"); pc = pc + 1; },
            0x94 => { println!("SUB     H"); pc = pc + 1; },
            0x95 => { println!("SUB     L"); pc = pc + 1; },
            0x96 => { println!("SUB     M"); pc = pc + 1; },
            0x97 => { println!("SUB     A"); pc = pc + 1; },
            0x98 => { println!("SBB     B"); pc = pc + 1; },
            0x99 => { println!("SBB     C"); pc = pc + 1; },
            0x9a => { println!("SBB     D"); pc = pc + 1; },
            0x9b => { println!("SBB     E"); pc = pc + 1; },
            0x9c => { println!("SBB     H"); pc = pc + 1; },
            0x9d => { println!("SBB     L"); pc = pc + 1; },
            0x9e => { println!("SBB     M"); pc = pc + 1; },
            0x9f => { println!("SBB     A"); pc = pc + 1; },
            0xa0 => { println!("ANA     B"); pc = pc + 1; },
            0xa1 => { println!("ANA     C"); pc = pc + 1; },
            0xa2 => { println!("ANA     D"); pc = pc + 1; },
            0xa3 => { println!("ANA     E"); pc = pc + 1; },
            0xa4 => { println!("ANA     H"); pc = pc + 1; },
            0xa5 => { println!("ANA     L"); pc = pc + 1; },
            0xa6 => { println!("ANA     M"); pc = pc + 1; },
            0xa7 => { println!("ANA     A"); pc = pc + 1; },
            0xa8 => { println!("XRA     B"); pc = pc + 1; },
            0xa9 => { println!("XRA     C"); pc = pc + 1; },
            0xaa => { println!("XRA     D"); pc = pc + 1; },
            0xab => { println!("XRA     E"); pc = pc + 1; },
            0xac => { println!("XRA     H"); pc = pc + 1; },
            0xad => { println!("XRA     L"); pc = pc + 1; },
            0xae => { println!("XRA     M"); pc = pc + 1; },
            0xaf => { println!("XRA     A"); pc = pc + 1; },










































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
