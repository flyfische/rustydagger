#![feature(augmented_assignments)]
use std::fs::File;
use std::io::prelude::*;
use std::env;

mod disassemble;
mod vm;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let mut rom = File::open(arg1).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        let block = rom.read_to_end(&mut buffer);
        println!("Read {} bytes from file", block.unwrap());
        let mut vm = vm::Vm::new();
        vm.load_rom(buffer);
//        println!("Vm: {:#?}", vm);
        vm.run();
        //disassemble::disassemble(buffer);
    }
}
