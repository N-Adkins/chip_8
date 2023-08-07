mod emulator;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Must provide rom as command line argument");
    }

    let mut cpu = emulator::Cpu::new();
    
    cpu.load_rom(args[1].as_str());

    loop {
        cpu.decode_next_instruction();
        if cpu.dt > 0 { cpu.dt -= 1; }
        cpu.renderer.poll();
        cpu.renderer.render();
        ::std::thread::sleep(std::time::Duration::from_micros(1000));
    }


}
