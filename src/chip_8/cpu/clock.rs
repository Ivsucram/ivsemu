use std::time::SystemTime;

pub struct Clock {
    pub tick: u8,
    pub clock_hz: f64,
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

    pub fn reset_elapsed(&mut self) {
        self.elapsed = SystemTime::now();
    }

    pub fn clock_hz_as_secs_f64(&self) -> f64 {
        1. / self.clock_hz
    }
}
