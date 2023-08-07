use super::instruction::Instruction;
use super::memory::Memory;
use super::stack::Stack;
use super::renderer::Renderer;

use std::io;
use std::io::prelude::*;
use std::fs::File;

extern crate rand;
use rand::Rng;

pub struct Cpu {
    pc: u16,
    pub dt: u8,
    i: u16,
    rng: rand::rngs::ThreadRng,
    memory: Memory,
    stack: Stack,
    pub renderer: Renderer,
    registers: [u8; 16],
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            pc: 0x200,
            dt: 0,
            i: 0,
            rng: rand::thread_rng(),
            memory: Memory::new(),
            stack: Stack::new(),
            renderer: Renderer::new(),
            registers: [0; 16],
        }
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        self.memory.load_rom(&buffer);
    }

    pub fn decode_next_instruction(&mut self) {
        let opcode = self.memory.read_u16(self.pc);
        self.pc += 2;
        self.execute_instruction(&Instruction::new(opcode));
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        //println!("Executing instruction: {:#06X?}", instruction.raw);
        match instruction.raw & 0xF000 {
            0x0000 => match instruction.raw {
                0x00E0 => self.op_cls(),
                0x00EE => self.op_ret(),
                _ => println!("SYS addr instruction, ignoring"), 
            }
            0x1000 => self.op_jp_addr(&instruction),
            0x2000 => self.op_call_addr(&instruction),
            0x3000 => self.op_se_vx_byte(&instruction),
            0x4000 => self.op_sne_vx_byte(&instruction),
            0x5000 => self.op_se_vx_vy(&instruction),
            0x6000 => self.op_ld_vx_byte(&instruction),
            0x7000 => self.op_add_vx_byte(&instruction),
            0x8000 => match instruction.raw & 0x000F {
                0x0000 => self.op_ld_vx_vy(&instruction),
                0x0001 => self.op_or_vx_vy(&instruction),
                0x0002 => self.op_and_vx_vy(&instruction), 
                0x0003 => self.op_xor_vx_vy(&instruction),
                0x0004 => self.op_add_vx_vy(&instruction),
                0x0005 => self.op_sub_vx_vy(&instruction),
                0x0006 => self.op_shr_vx_vy(&instruction),
                0x0007 => self.op_subn_vx_vy(&instruction),
                0x000E => self.op_shl_vx_vy(&instruction),
                _ => panic!("Unhandled instruction: {:#06X?}", instruction.raw),
            }
            0x9000 => self.op_sne_vx_vy(&instruction),
            0xA000 => self.op_ld_i_addr(&instruction),
            0xB000 => self.op_jp_v0_addr(&instruction),
            0xC000 => self.op_rnd_vx_byte(&instruction),
            0xD000 => self.op_drw_vx_vy_n(&instruction),
            0xE000 => match instruction.raw & 0x00FF {
                0x9E => self.op_skp_vx(&instruction),
                0xA1 => self.op_sknp_vx(&instruction),
                _ => panic!("Unhandled instruction: {:#06X?}", instruction.raw),
            }
            0xF000 => match instruction.raw & 0x00FF {
                0x07 => self.op_ld_vx_dt(&instruction),
                0x15 => self.op_ld_dt_vx(&instruction),
                0x18 => self.op_ld_st_vx(&instruction),
                0x1E => self.op_add_i_vx(&instruction),
                0x29 => self.op_ld_f_vx(&instruction),
                0x33 => self.op_ld_b_vx(&instruction),
                0x55 => self.op_ld_i_vx(&instruction),
                0x65 => self.op_ld_vx_i(&instruction),
                _ => panic!("Unhandled instruction: {:#06X?}", instruction.raw),
            }
            _ => panic!("Unhandled instruction: {:#06X?}", instruction.raw),
        }
    }
    
    fn op_cls(&mut self) {
        self.renderer.clear_display();
    }

    fn op_ret(&mut self) {
        self.pc = self.stack.pop();
    }

    fn op_jp_addr(&mut self, instruction : &Instruction) {
        self.pc = instruction.nnn();
    }

    fn op_call_addr(&mut self, instruction: &Instruction) {
        self.stack.push(self.pc);
        self.pc = instruction.nnn();
    }

    fn op_se_vx_byte(&mut self, instruction: &Instruction) {
        if self.registers[instruction.x()] == instruction.kk() {
            self.pc += 2;
        }
    }

    fn op_sne_vx_byte(&mut self, instruction: &Instruction) {
        if self.registers[instruction.x()] != instruction.kk() {
            self.pc += 2;
        }
    }

    fn op_se_vx_vy(&mut self, instruction: &Instruction) {
        let register_x = self.registers[instruction.x()];
        let register_y = self.registers[instruction.y()];
        if register_x == register_y {
            self.pc += 2;
        }
    }

    fn op_ld_vx_byte(&mut self, instruction: &Instruction) {
        self.registers[instruction.x()] = instruction.kk();
    }

    fn op_add_vx_byte(&mut self, instruction: &Instruction) {
        let result = self.registers[instruction.x()].wrapping_add(instruction.kk());
        self.registers[instruction.x()] = result;
    }

    fn op_ld_vx_vy(&mut self, instruction: &Instruction) {
        self.registers[instruction.x()] = self.registers[instruction.y()];
    }

    fn op_or_vx_vy(&mut self, instruction: &Instruction) {
        let result = self.registers[instruction.x()] | self.registers[instruction.y()];
        self.registers[instruction.x()] = result;
    }

    fn op_and_vx_vy(&mut self, instruction: &Instruction) {
        let result = self.registers[instruction.x()] & self.registers[instruction.y()];
        self.registers[instruction.x()] = result;
    }

    fn op_xor_vx_vy(&mut self, instruction: &Instruction) {
        let result = self.registers[instruction.x()] ^ self.registers[instruction.y()];
        self.registers[instruction.x()] = result;
    }

    fn op_add_vx_vy(&mut self, instruction: &Instruction) {
        let register_x = self.registers[instruction.x()];
        let register_y = self.registers[instruction.y()];
        let register_x_u16 = register_x as u16;
        let register_y_u16 = register_y as u16;
        let result = register_x.wrapping_add(register_y);                   
        self.registers[0xF] = ((result as u16) < register_x_u16 + register_y_u16) as u8;
        self.registers[instruction.x()] = result;
    }

    fn op_sub_vx_vy(&mut self, instruction: &Instruction) {
        let register_x = self.registers[instruction.x()];
        let register_y = self.registers[instruction.y()];
        self.registers[0xF] = (register_x > register_y) as u8;
        self.registers[instruction.x()] = register_x.wrapping_sub(register_y);
    }

    fn op_shr_vx_vy(&mut self, instruction: &Instruction) {
        let register_x = self.registers[instruction.x()];
        self.registers[0xF] = register_x & 0x1;
        self.registers[instruction.x()] = register_x.wrapping_div(2);
    }

    fn op_subn_vx_vy(&mut self, instruction: &Instruction) {
        let register_x = self.registers[instruction.x()];
        let register_y = self.registers[instruction.y()];
        self.registers[0xF] = (register_y > register_x) as u8;
        self.registers[instruction.x()] = register_y.wrapping_sub(register_x);
    }

    fn op_shl_vx_vy(&mut self, instruction: &Instruction) {
        let register_x = self.registers[instruction.x()];
        self.registers[0xF] = (register_x & 0x80) >> 7;
        self.registers[instruction.x()] = register_x.wrapping_mul(2);
    }
    
    fn op_sne_vx_vy(&mut self, instruction: &Instruction) {
        if self.registers[instruction.x()] != self.registers[instruction.y()] {
            self.pc += 2;
        }
    }

    fn op_ld_i_addr(&mut self, instruction: &Instruction) {
        self.i = instruction.nnn();
    }

    fn op_jp_v0_addr(&mut self, instruction: &Instruction) {
        self.pc = instruction.nnn() + self.registers[0x0] as u16;
    }

    fn op_rnd_vx_byte(&mut self, instruction: &Instruction) {
        self.registers[instruction.x()] = self.rng.gen_range(0..255) & instruction.kk();
    }

    fn op_drw_vx_vy_n(&mut self, instruction: &Instruction) {
        
        let coord_x = self.registers[instruction.x()] as u16;
        let coord_y = self.registers[instruction.y()] as u16;
        let height = instruction.n();

        self.registers[0xF] = 0;
        
        for yline in 0..height {

            let pixel = self.memory.read_u8(self.i + yline);

            for xline in 0..8 {

                if pixel & (0x80 >> xline) != 0 {
                    
                    let index = (coord_x + xline + (coord_y + yline) * 64) as usize;

                    if index > 2047 { continue; }

                    if self.renderer.display[index] > 0 {
                         self.registers[0xF] = 1;
                    }

                    self.renderer.display[index] ^= 255;

                }

            }

        }

        self.renderer.update_texture();

    }

    fn op_skp_vx(&mut self, instruction: &Instruction) {
        if self.renderer.keys[self.registers[instruction.x()] as usize] == 1 {
            self.pc += 2; 
        }
    }

    fn op_sknp_vx(&mut self, instruction: &Instruction) {
        if self.renderer.keys[self.registers[instruction.x()] as usize] == 0 {
            self.pc += 2;
        }
    }

    fn op_ld_vx_dt(&mut self, instruction: &Instruction) {
        self.registers[instruction.x()] = self.dt;
    }

    fn op_ld_dt_vx(&mut self, instruction: &Instruction) {
        self.dt = self.registers[instruction.x()];
    }

    fn op_ld_i_vx(&mut self, instruction: &Instruction) {
        for register in 0..instruction.x() {
            self.memory.set_u8(self.i + register as u16, self.registers[register]);
        }
    }

    fn op_ld_vx_i(&mut self, instruction: &Instruction) {
        for register in 0..instruction.x() {
            self.registers[register] = self.memory.read_u8(self.i + register as u16);
        }
    }

    fn op_ld_st_vx(&mut self, _instruction: &Instruction) {
        println!("Sound instruction, ignoring");
    }

    fn op_add_i_vx(&mut self, instruction: &Instruction) {
        self.i += self.registers[instruction.x()] as u16;
    }

    fn op_ld_f_vx(&mut self, instruction: &Instruction) {
        self.i = (0x50 + self.registers[instruction.x()] * 5) as u16;
    }

    fn op_ld_b_vx(&mut self, instruction: &Instruction) {
        let mut register = self.registers[instruction.x()];
        let ones: u8 = register % 10;
        register = register / 10;
        let tens: u8 = register / 10;
        let hundreds: u8 = register % 10;
        self.memory.set_u8(self.i, hundreds);
        self.memory.set_u8(self.i + 1, tens);
        self.memory.set_u8(self.i + 2, ones);
    }
    
}
