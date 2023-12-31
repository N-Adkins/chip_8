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

        cpu.renderer.poll();

        cpu.renderer.render();

        if cpu.dt > 0 { cpu.dt -= 1; }
        if cpu.st > 0 { 
            cpu.renderer.audio.device.resume(); 
            cpu.st -= 1; 
        } else {
            cpu.renderer.audio.device.pause();
        }

        for _ in 0..20 {
            cpu.decode_next_instruction();
        }

        cpu.drawn_this_frame = false;

        cpu.renderer.update_texture();

        ::std::thread::sleep(std::time::Duration::from_secs_f64(1.0/60.0))

    }


}
