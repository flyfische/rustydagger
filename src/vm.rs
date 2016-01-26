#![allow(dead_code)]
use std::io::Write;
use std::time::Duration;
use std::thread;
use std::num::Wrapping;
/*
    This is the implementation of the VM itself.  
    I'm testing this on the Space Invaders rom, which only uses around 50 of the 256 instructions. 
    Therefore, they'll probably be implemented first.
*/
#[derive(Debug, Default)]
struct ConditionCodes {
    // Condition codes
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
    pad: u8,
}
#[derive(Debug, Default)]
pub struct Vm {
    // State
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: usize,            // Stack Pointer
    pc: usize,            // Program Counter
    int_enable: u8,
    memory: Vec<u8>,
    condition_codes: ConditionCodes,
}

impl Vm {
    fn run_current_opcode(&mut self) {
        let opcode: u8 = self.memory[self.pc];
        match opcode {
            0x00 => { self.pc += 1;  }, 
            0x01 => { 
                // LXI B, D16
                self.c = self.memory[self.pc + 1];
                self.b = self.memory[self.pc + 2];
                self.pc += 3;
            },
            0xc3 => {
                // JMP addr
                let mut new_pc: u16 = (self.memory[self.pc + 2] as u16) << 8;
                new_pc = new_pc | (self.memory[self.pc + 1] as u16);
                self.pc =  new_pc as usize;
            },
            0x31 => {
                // LXI SP, D16
                let mut new_pc: u16 = (self.memory[self.pc + 2] as u16) << 8;
                new_pc = new_pc | (self.memory[self.pc + 1] as u16);
                self.sp = new_pc as usize;
                self.pc += 3;
            },
            0x06 => {
                // MVI B, D8
                self.b = self.memory[self.pc + 1];
                self.pc += 2;
            },
            0xcd => {
                // CALL
                let mut new_pc: u16 = (self.memory[self.pc + 2] as u16) << 8;
                new_pc = new_pc | (self.memory[self.pc + 1] as u16);
                let ret_addr: u16 = (self.sp + 2) as u16;

                self.memory[self.sp - 1] = ((ret_addr >> 8) & 0xff) as u8;
                self.memory[self.sp - 2] = (ret_addr & 0xff) as u8;
                self.sp -= 2;
                self.pc = new_pc as usize;
            },
            0x11 => {
                // LXI D, D16
                self.e = self.memory[self.pc + 1];
                self.d = self.memory[self.pc + 2];
                self.pc += 3;
            },
            0x21 => {
                // LXI H, D16
                self.l = self.memory[self.pc + 1];
                self.h = self.memory[self.pc + 2];
                self.pc += 3;
            },
            0x1a => {
                // LDAX D
                let mut offset: u16 = (self.d as u16) << 8;
                offset = offset | (self.e as u16);
                self.a = self.memory[offset as usize];
                self.pc += 1;

            },
            0x77 => {
                // MOVE M,A
                let mut offset: u16 = (self.h as u16) << 8;
                offset = offset | (self.l as u16);
                self.memory[offset as usize] = self.a;
                self.pc += 1;
            },
            0x23 => {
                // INX H
                self.l += 1;
                if self.l == 0 {
                    self.h += 1;
                }
                self.pc += 1;
            },
            0x13 => {
                // INX D
                self.e += 1;
                if self.e == 0 {
                    self.d += 1;
                }
                self.pc += 1;
            },
            0x05 => {
                // DCR B
                let mut res: u8 = (Wrapping(self.b) - Wrapping(1)).0;
                self.condition_codes.z = (res == 0) as u8;
                self.condition_codes.s = (0x80 == (res & 0x80)) as u8;
                self.condition_codes.p = self.parity(res);
                self.b = res;
                self.pc += 1;
            },
            0xc2 => {
                let mut new_pc: u16 = (self.memory[self.pc + 2] as u16) << 8;
                new_pc = new_pc | (self.memory[self.pc + 1] as u16);
                if self.condition_codes.z == 0 {
                    self.pc = new_pc as usize;
                }
                else {
                    self.pc += 2;
                }
            },

           _ => { self.unimplemented_instruction(opcode); }

        }
    }
    fn parity(&mut self, mut res: u8) -> u8 {
        let mut p: i32 = 0;
        for x in 0..8 {
            if (x & 0x1) == 1 {
                p += 1;
            }
            res = res >> 1;
        }
        (0 == (p & 0x1)) as u8
    }
    fn unimplemented_instruction(&self, opcode: u8) {
        panic!("Reached unimplemented opcode: 0x{}", format!("{:01$x}", opcode, 2));
    }
   pub fn new() -> Vm {
       Vm {
           memory: vec![0; 10000], ..Default::default()
       }
   }
   pub fn load_rom(&mut self, mut rom: Vec<u8>) {
       self.memory[..rom.len()].clone_from_slice(&mut rom);
       println!("size: {}", self.memory.len());
   }
    pub fn run(&mut self) {
        loop {
            self.run_current_opcode();
            println!("{} => {}", format!("{:01$x}", self.pc, 4), 
                     format!("{:01$x}", self.memory[self.pc], 2));
//            thread::sleep(Duration::from_millis(300));
        }
    }

}

