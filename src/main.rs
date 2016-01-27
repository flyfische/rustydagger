#![feature(augmented_assignments)]

extern crate ncurses;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::time::Duration;
use std::thread;
use ncurses::*;
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
        initscr();        
        loop {
            vm.run_current_opcode();
            mvprintw(0,0,format!("{:#?}", vm).as_str()); 
            thread::sleep(Duration::from_millis(300));
            refresh();
        }
    }
}
