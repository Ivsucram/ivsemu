use std::time::SystemTime;

pub struct Clock {
    pub tick: u8,
    clock_hz: f64,
    elapsed: SystemTime,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            tick: 0xFF,
            clock_hz: 60.,
            elapsed: SystemTime::now(),
        }
    }

    pub fn tick(&mut self) -> bool {
        let mut res: bool = false;
        match self.elapsed.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs_f64() >= self.clock_hz_as_secs_f64() {
                    if self.tick > 0 {
                        self.tick -= 1;
                    }
                    self.reset_elapsed();
                    res = true;
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        res
    }

    fn reset_elapsed(&mut self) {
        self.elapsed = SystemTime::now();
    }

    pub fn clock_hz_as_secs_f64(&self) -> f64 {
        1. / self.clock_hz
    }

    pub fn increase_clock(&mut self, is_printing: bool) {
        if is_printing {
            println!(
                "Increasing cpu clock from {:5} Hz to {:5} Hz",
                self.clock_hz,
                self.clock_hz + 10.
            );
        }
        self.clock_hz += 10.;
    }

    pub fn decrease_clock(&mut self, is_printing: bool) {
        if self.clock_hz > 10. {
            if is_printing {
                println!(
                    "Decreasing cpu clock from {:5} Hz to {:5} Hz",
                    self.clock_hz,
                    self.clock_hz - 10.
                );
            }
            self.clock_hz -= 10.;
        }
    }
}
