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
//        panic!("instruction: {}", vm.memory[0x1a3a]);
        initscr();        
        let mut cycles: i32 = 0;
        loop {
        
            vm.run_current_opcode();
            if  (time::precise_time_s() - last_interrupt) > 1.0/60.0  {
                if vm.int_enable == 1 {
                    //vm.generate_interrupt(interrupt_num);
                    last_interrupt = time::precise_time_s();
                }
            }
            mvprintw(0,0,format!("{:#?}", vm).as_str()); 
//            thread::sleep(Duration::from_millis(5));
            refresh();
            cycles += 1;
            mvprintw(25,0,format!("{}", cycles).as_str());
            if cycles >= 420000 {
                let mut file = File::create("screen.bmp").unwrap();
                file.write_all(&vm.memory[0x2400..0x4000]);
                getch();
            }
        }
    }
}
