#![feature(augmented_assignments)]

extern crate ncurses;
extern crate time;

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
        println!("Time: {}", time::precise_time_s());
        let mut last_interrupt: f64 = 0.0;
        let mut interrupt_num: i32 = 2;
        initscr();        
        loop {
            vm.run_current_opcode();
            if  (time::precise_time_s() - last_interrupt) > 1.0/60.0  {
                if vm.int_enable == 1 {
                    vm.generate_interrupt(interrupt_num);
                    last_interrupt = time::precise_time_s();
                }
            }
            mvprintw(0,0,format!("{:#?}", vm).as_str()); 
//            thread::sleep(Duration::from_millis(50));
            refresh();
        }
    }
}
