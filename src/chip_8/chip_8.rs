use super::cpu::cpu::CPU;
use super::display::Display;

use std::io::Read;

pub fn run() {
    let mut cpu = CPU::new();
    load_rom("src/chip_8/roms/CAVE.ch8", &mut cpu);

    let sdl_context = sdl2::init().unwrap();
    let mut display = Display::init(&sdl_context, cpu.get_display_scale());
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
                    cpu.increase_clock(true);
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::LeftBracket),
                    ..
                } => {
                    cpu.decrease_clock(true);
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Backspace),
                    ..
                } => {
                    cpu.reset_rom();
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key_index) = cpu.compute_keycode(keycode) {
                        cpu.press_key(key_index);
                    }
                }
                sdl2::event::Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key_index) = cpu.compute_keycode(keycode) {
                        cpu.release_key(key_index);
                    }
                }
                _ => {}
            }
        }

        cpu.tick_delay_timer();
        cpu.tick_sound_timer();
        if cpu.tick() {
            cpu.fetch();
            cpu.decode();
            if cpu.should_redraw {
                display.draw(&cpu.get_display_buffer());
                cpu.should_redraw = false;
            }
        }
    }
}

fn load_rom(filename: &str, cpu: &mut CPU) {
    let mut f = std::fs::File::open(&filename).expect("no file found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    cpu.load_rom(&buffer);
}
