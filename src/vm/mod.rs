#![allow(dead_code)]
use std::time::Duration;
use std::thread;
use std::num::Wrapping;
use std::fmt;
/*
    This is the implementation of the VM itself.  
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
#[derive(Default)]
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
    pub int_enable: u8,
    pub memory: Vec<u8>,
    condition_codes: ConditionCodes,
}
fn format(hex: u8) -> String {
    format!("{:01$x}", hex, 2)
}
fn format_usize(hex: usize) -> String {
    format!("{:01$x}", hex, 4)
}

impl fmt::Debug for Vm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vm {{\n\t a: {}\n\t b: {}\n\t c: {}\n\t d: {}\n\t e: {}\n\t h: {}\n\t \
        l: {}\n\t sp: {}\n\t pc: {}\n\t int_enable: {}\n\t condition_codes:\n\t {:#?}\n }}",
        format(self.a), format(self.b), format(self.c), format(self.d), format(self.e),
        format(self.h), format(self.l), format_usize(self.sp),
        format_usize(self.pc), format(self.int_enable), self.condition_codes)
    }
}
impl Vm {
    pub fn run_current_opcode(&mut self) {
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
                self.pc =  self.merge_addr_pair(self.memory[self.pc + 1],
                                                self.memory[self.pc + 2]) as usize;
            },
            0x31 => {
                // LXI SP, D16
                self.sp = self.merge_addr_pair(self.memory[self.pc + 1],
                                               self.memory[self.pc + 2]) as usize;
                self.pc += 3;
            },
            0x06 => {
                // MVI B, D8
                self.b = self.memory[self.pc + 1];
                self.pc += 2;
            },
            0xcd => {
                // CALL
                let ret_addr: u16 = (self.pc + 3) as u16;
                self.memory[self.sp - 1] = ((ret_addr >> 8) & 0xff) as u8;
                self.memory[self.sp - 2] = (ret_addr & 0xff) as u8;
                self.sp -= 2;
                self.pc = self.merge_addr_pair(self.memory[self.pc+1],
                                               self.memory[self.pc+2]) as usize;
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
                self.l = self.floating_point_add(self.l, 1);
                if self.l == 0 {
                    self.h += 1;
                }
                self.pc += 1;
            },
            0x13 => {
                // INX D
                self.e = self.floating_point_add(self.e,1);
                if self.e == 0 {
                    self.d += 1;
                }
                self.pc += 1;
            },
            0x05 => {
                // DCR B
                
                let res: u8 = self.floating_point_sub(self.b, 1);
                self.condition_codes.z = (res == 0) as u8;
                self.condition_codes.s = (0x80 == (res & 0x80)) as u8;
                self.condition_codes.p = self.parity(res);
                self.b = res;
                self.pc += 1;
            },
            0xc2 => {
                // JNZ
                if self.condition_codes.z == 0 {
                    self.pc = self.merge_addr_pair(self.memory[self.pc+1], 
                                                   self.memory[self.pc+2]) as usize;
                }
                else {
                    self.pc += 3;
                }
            },
            0xc9 => {
                // RET
                self.pc = self.merge_addr_pair(self.memory[self.sp],
                                               self.memory[self.sp + 1]) as usize;
                self.sp += 2;
            },
            0x19 => {
                // DAD D
                let hl: u32 = ((self.h as u32) << 8) | (self.l as u32);
                let de: u32 = ((self.d as u32) << 8) | (self.e as u32);
                let res: u32 = hl + de;
                self.h = ((res & 0xff00) >> 8) as u8;
                self.l = (res & 0xff) as u8;
                self.condition_codes.cy = ((res & 0xffff0000) != 0) as u8;
                self.pc += 1;
            },
            0x3e => {
                // MVI A, D8
                self.a = self.memory[self.pc + 1];
                self.pc += 2;
            },
            0x32 => {
                // STA word
                let offset: u16 = self.merge_addr_pair(self.memory[self.pc + 1],
                                                       self.memory[self.pc + 2]);
                self.memory[offset as usize] = self.a;
                self.pc += 3;
            },
            0xaf => {
                // XRA A
                self.a = self.a ^ self.a;
                self.logic_flags_a();
                self.pc += 1;
            },
            0xd3 => {
                // OUT D8
                // NOT IMPLEMENTED YET
                println!("OUT: {}", self.memory[self.pc + 1]);
                self.pc += 2;
            },
            0xfb => {
                // EI
                // Enable interrupt?
                self.int_enable = 1;
                self.pc += 1;
            },
            0x3a => {
                // LDA word
                let offset: u16 = self.merge_addr_pair(self.memory[self.pc + 1],
                                                       self.memory[self.pc + 2]);
                self.a = self.memory[offset as usize];
                self.pc += 3;
            },
            0xa7 => {
                // ANA A
                self.a = self.a & self.a;
                self.logic_flags_a();
                self.pc += 1;
            },
            0xf5 => {
                self.memory[self.sp - 1] = self.a;
                let psw: u8 =  self.condition_codes.z |
                                 self.condition_codes.s << 1 |
                                 self.condition_codes.p << 2 |
                                 self.condition_codes.cy << 3 |
                                 self.condition_codes.ac << 4 ;
                self.memory[self.sp - 2] = psw as u8;
                self.sp -= 2;
                self.pc += 1;
            },
            0xc5 => {
                self.memory[self.sp - 1] = self.b;
                self.memory[self.sp - 2] = self.c;
                self.sp -= 2;
                self.pc += 1;
            },
            0xd5 => {
                self.memory[self.sp - 1] = self.d;
                self.memory[self.sp - 2] = self.e;
                self.sp -= 2;
                self.pc += 1;
            },
            0xe5 => {
                self.memory[self.sp - 1] = self.h;
                self.memory[self.sp - 2] = self.l;
                self.sp -= 2;
                self.pc += 1;
            },
            0x35 => {
                let mut res: u8 = self.read_from_hl();
                res = self.floating_point_add(res, 1);
                self.zsp_flags(res);
                self.write_to_hl(res);
                self.pc += 1;
            },
            0xdb => {
                let port = self.memory[self.pc + 1];
                if port == 2 {
                    self.a = 0;
                }
                else if port == 1 {
                    self.a = 0;
                }
                else {
                    panic!("Found port: {}", port);
                }
                self.pc += 2;
            },
            0xe6 => {
                self.a = self.a & self.memory[self.pc + 1];
                self.logic_flags_a();
                self.pc += 2;
            },
            0xc8 => {
                if self.condition_codes.z == 1 {
                    self.pc = ((self.memory[self.sp] as u16) |
                        ((self.memory[self.sp + 1] as u16) << 8)) as usize;
                    self.sp += 2;
                   
                }
                 self.pc += 1;
            },
            0x0f => {
                let x: u8 = self.a;
                self.a = ((x & 1) << 7) | ( x >> 1);
                self.condition_codes.cy = (1 == (x & 1)) as u8;
                self.pc += 1;
            },
            0xda => {
                if self.condition_codes.cy != 0 {
                    self.pc = self.merge_addr_pair(self.memory[self.pc + 1], 
                                                   self.memory[self.pc + 2]) as usize;
                }
                else {
                    self.pc += 3;
                }

            },
            0xca => {
                if self.condition_codes.z == 1 {
                    self.pc = self.merge_addr_pair(self.memory[self.pc + 1], 
                                                   self.memory[self.pc + 2]) as usize;
                }
                else {
                    self.pc += 3;
                }
            },
            0x0a => {
                let offset: u16 = self.merge_addr_pair(self.c, self.b);
                self.a = self.memory[offset as usize];
                self.pc += 1;
            },
            0x36 => {
                let offset: u16 = ((self.h as u16 )<< 8) | (self.l as u16);
                self.memory[offset as usize] = self.memory[self.pc + 1];
                self.pc += 2;
            },
            0x7c => {
                self.a = self.h;
                self.pc += 1;
            }, 
            0xfe => {
                let x: u8 = self.floating_point_sub(self.a, self.memory[self.pc + 1]) as u8;
                self.condition_codes.z = (x == 0) as u8;
                self.condition_codes.s = (0x80 == (x & 0x80)) as u8;
                self.condition_codes.p = self.parity(x);
                self.condition_codes.cy = (self.a < self.memory[self.pc + 1]) as u8;
                self.pc += 2;
            },
            0x0e => {
                self.c = self.memory[self.pc + 1];
                self.pc += 2;
            },
            0x26 => {
                self.h = self.memory[self.pc + 1];
                self.pc += 2;
            },
            0x6f => {
                self.l = self.a;
                self.pc += 1;
            },
            0x29 => {
                let hl: u32 = ((self.h as u32) << 8) | (self.l as u32);
                let res: u32 = hl + hl;
                self.h = ((res & 0xff00) >> 8) as u8;
                self.l = (res & 0xff) as u8;
                self.condition_codes.cy = ((res & 0xffff0000) != 0) as u8;
                self.pc += 1;

            },
            0xeb => {
                let d: u8 = self.d;
                let e: u8 = self.e;
                self.d = self.h;
                self.e = self.l;
                self.h = d;
                self.l = e;
                self.pc += 1;
            },
            0xe1 => {
                self.l = self.memory[self.sp];
                self.h = self.memory[self.sp + 1];
                self.sp += 2;
                self.pc += 1;
            },
            0x09 => {
                let hl: u32 = ((self.h as u32) << 8) | (self.l as u32);
                let bc: u32 = ((self.b as u32) << 8) | (self.c as u32);
                let res = hl + bc;
                self.h = ((res & 0xff00) >> 8) as u8;
                self.l = (res & 0xff) as u8;
                self.condition_codes.cy = ((res & 0xffff0000) > 0) as u8;
                self.pc += 1;
            },
            0xc1 => {
                self.c = self.memory[self.sp];
                self.b = self.memory[self.sp + 1];
                self.sp += 2;
                self.pc += 1;
            },
            0xd1 => {
                self.e = self.memory[self.sp];
                self.d = self.memory[self.sp + 1];
                self.sp += 2;
                self.pc += 1;
            },
            0x0d => {
                let res: u8 = self.floating_point_sub(self.c, 1);
                self.condition_codes.z = (res == 0) as u8;
                self.condition_codes.s = (0x80 == (res & 0x80)) as u8;
                self.condition_codes.p = self.parity(res);
                self.c = res;
                self.pc += 1;
            },
            0x5e => {
                let offset: u16 = self.merge_addr_pair(self.l, self.h);
                self.e = self.memory[offset as usize];
                self.pc += 1;
            },
            0x56 => {
                let offset: u16 = self.merge_addr_pair(self.l, self.h);
                self.d = self.memory[offset as usize];
                self.pc += 1;
            },
            0x7e => {
                let offset: u16 = self.merge_addr_pair(self.l, self.h);
                self.a = self.memory[offset as usize];
                self.pc += 1;
            },
            0x66 => {
                let offset: u16 = self.merge_addr_pair(self.l, self.h);
                self.h = self.memory[offset as usize];
                self.pc += 1;
            },
            0x7a => {
                self.a = self.d;
                self.pc += 1;
            },
            0xc6 => {
                let x: u16 = self.floating_point_add(self.a, self.memory[self.pc + 1]) as u16;
                self.condition_codes.z = ((x & 0xff) == 0) as u8;
                self.condition_codes.s = (0x80 == (x & 0x80)) as u8;
                self.condition_codes.p = self.parity((x & 0xff) as u8);
                self.condition_codes.cy = (x > 0xff) as u8;
                self.a = x as u8;
                self.pc += 2;
            },
            0xf1 => {
                self.a = self.memory[self.sp + 1];
                let psw: u8 = self.memory[self.sp];
                self.condition_codes.z = (0x01 == (psw & 0x01)) as u8;
                self.condition_codes.s = (0x02 == (psw & 0x02)) as u8;
                self.condition_codes.p = (0x04 == (psw & 0x04)) as u8;
                self.condition_codes.cy = (0x05 == (psw & 0x05)) as u8;
                self.condition_codes.ac = (0x10 == (psw & 0x10)) as u8;
                self.sp += 2;
                self.pc += 1;
            },
            0x7b => {
                self.a = self.e;
                self.pc += 1;
            },
                        
            
           _ => { self.unimplemented_instruction(opcode); }

        }
    }
    fn zsp_flags(&mut self, value: u8) {
        self.condition_codes.z = (value == 0) as u8;
        self.condition_codes.s = (0x80 == (value & 0x80)) as u8;
        self.condition_codes.p = self.parity(value);
    }
    fn read_from_hl(&self) -> u8 {
        let offset: u16 = ((self.h as u16) << 8) | (self.l as u16);
        self.memory[offset as usize]
    }
    fn write_to_hl(&mut self, value: u8) {
        let offset: u16 = ((self.h as u16) << 8) | (self.l as u16);
        self.memory[offset as usize] = value;
    }
    pub fn generate_interrupt(&mut self, interrupt_num: i32) {
        let pc: u16 = self.pc as u16;
        self.push_stack(((pc & 0xff00) >> 8) as u8, (pc & 0xff) as u8);
        self.pc = (8 * interrupt_num) as usize;
        self.int_enable = 0;
        println!("Interrupt!");
    }
    fn merge_addr_pair(&self, lo: u8, hi: u8) -> u16 {
        let mut new_addr: u16 = (hi as u16) << 8;
        new_addr = new_addr | (lo as u16);
        new_addr
    }
    fn push_stack(&mut self, hi: u8, lo: u8) {
        self.memory[self.sp - 1] = hi;
        self.memory[self.sp - 2] = lo;
        self.sp -= 2;
    }
    fn floating_point_add(&self, x: u8, y:u8) -> u8 {
        // We need to wrap these to allow overflows
        (Wrapping(x) + Wrapping(y)).0
    }
    fn floating_point_sub(&self, x: u8, y: u8) -> u8 {
        // We need to wrap these to allow overflows
        (Wrapping(x) - Wrapping(y)).0
    }
    pub fn parity(&mut self, mut res: u8) -> u8 {
        let mut p: i32 = 0;
        for x in 0..8 {
            if (x & 0x1) == 1 {
                p += 1;
            }
            res = res >> 1;
        }
        (0 == (p & 0x1)) as u8
    }
    fn logic_flags_a(&mut self) {
        self.condition_codes.cy = 0;
        self.condition_codes.ac = 0;
        self.condition_codes.z = (self.a == 0) as u8;
        self.condition_codes.s = (0x80 == (self.a & 0x80)) as u8;
        let a: u8 = self.a;
        self.condition_codes.p = self.parity(a);
    }
    fn unimplemented_instruction(&self, opcode: u8) {
        panic!("Reached unimplemented opcode: 0x{}", format!("{:01$x}", opcode, 2));
    }
   pub fn new() -> Vm {
       Vm {
           memory: vec![0; 64000], ..Default::default()
       }
   }
   pub fn load_rom(&mut self, mut rom: Vec<u8>) {
       self.memory[..rom.len()].clone_from_slice(&mut rom);
       println!("size: {}", self.memory.len());
   }
    pub fn print_debug(&self) {
        println!("{} => {}", format!("{:01$x}", self.pc, 4), 
             format!("{:01$x}", self.memory[self.pc], 2));
    }
    pub fn run(&mut self) {
        loop {
            self.run_current_opcode();
            println!("\r{:#?}", self);
            thread::sleep(Duration::from_millis(300));
        }
    }
}

