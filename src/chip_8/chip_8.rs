use super::cpu::cpu::CPU;
use super::display::Display;

use std::io::Read;

pub fn run() {
    let mut cpu = CPU::new();
    cpu.ram.init_fonts(); //TODO: load in the RAM
    cpu.clock.clock_hz = 60.;
    load_rom("src/chip_8/roms/CAVE.ch8", &mut cpu);

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::init(&sdl_context, cpu.db.scale);
    let mut event_listener = sdl_context.event_pump().unwrap();

    'runner: loop {
        for event in event_listener.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'runner,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'runner,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::RightBracket),
                    ..
                } => {
                    println!(
                        "Increasing cpu clock from {:5} Hz to {:5} Hz",
                        cpu.clock.clock_hz,
                        cpu.clock.clock_hz + 10.
                    );
                    cpu.clock.clock_hz += 10.;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::LeftBracket),
                    ..
                } => {
                    println!(
                        "Decreasing cpu clock from {:5} Hz to {:5} Hz",
                        cpu.clock.clock_hz,
                        cpu.clock.clock_hz - 10.
                    );
                    cpu.clock.clock_hz -= 10.;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Backspace),
                    ..
                } => {
                    cpu.pc.0 = 0x200;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key_index) = cpu.keypad.compute_keycode(keycode) {
                        cpu.keypad.press(key_index);
                    }
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key_index) = cpu.keypad.compute_keycode(keycode) {
                        cpu.keypad.release(key_index);
                    }
                }
                _ => {}
            }
        }

        cpu.dt.tick();
        cpu.st.tick();
        if cpu.clock.tick() {
            cpu.fetch();
            cpu.decode();
            if cpu.decode_match("D???") {
                display.draw(&cpu.db.db)
            }
        }
    }
}

fn load_rom(filename: &str, cpu: &mut CPU) {
    let mut f = std::fs::File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    cpu.ram.write_rom(&buffer);
}
