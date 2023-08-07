#![allow(dead_code)]

mod emulator;

fn main() -> Result<(), std::io::Error> {
    let mut cpu = emulator::Cpu::new();
    cpu.load_rom("roms/breakout.rom")?;
    loop {
        cpu.decode_next_instruction();
        if cpu.dt > 0 { cpu.dt -= 1; }
        cpu.renderer.poll();
        cpu.renderer.render();
        ::std::thread::sleep(std::time::Duration::from_micros(2500));
    }
}
