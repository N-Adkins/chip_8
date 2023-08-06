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
            i: 0,
            rng: rand::thread_rng(),
            memory: Memory::new(),
            stack: Stack::new(),
            renderer: Renderer::new(),
            registers: [0; 16],
        }
    }

    pub fn load_rom(&mut self, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;
        self.memory.load_rom(&buffer);
        Ok(())
    }

    pub fn decode_next_instruction(&mut self) {
        let opcode = self.memory.read_u16(self.pc);
        self.pc += 2;
        self.execute_instruction(&Instruction::new(opcode));
    }

    pub fn execute_instruction(&mut self, instruction: &Instruction) {
        println!("Executing instruction: {:#06X?}", instruction.raw);
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
        
        /*for n in 0..instruction.n() {
            let row = self.i + n;
            for i in 0..8 {
                if coord_x == 63 { break }
                let sprite_pixel_on = ((row >> i) & 0x1) == 0x1;
                let index = (coord_y * 64 + coord_x) as usize;
                if sprite_pixel_on && self.renderer.display[index] == 0x1 {
                    self.renderer.display[index] = 0;
                    self.registers[0xF] = 1;
                } else if sprite_pixel_on {
                    self.renderer.display[index] = 255; 
                }
                coord_x += 1;
            }
            coord_y += 1;
        }*/

        for yline in 0..height {

            let pixel = self.memory.read_u8(self.i + yline);

            for xline in 0..8 {

                if pixel & (0x80 >> xline) != 0 {
                    
                    if self.renderer.display[(coord_x + xline + (coord_y + yline) * 64) as usize] > 0 {
                         self.registers[0xF] = 1;
                    }

                    self.renderer.display[(coord_x + xline + (coord_y + yline) * 64) as usize] ^= 255;

                }

            }

        }

        self.renderer.update_texture();

    }
    
}
