#![allow(dead_code)]

mod emulator;

fn main() -> Result<(), std::io::Error> {
    let mut cpu = emulator::Cpu::new();
    cpu.load_rom("roms/test.rom")?;
    loop {
        cpu.decode_next_instruction();
    }
}
