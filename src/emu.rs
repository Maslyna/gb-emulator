#[derive(Debug)]
pub struct Emu {
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
}

impl Emu {
    pub fn new() -> Self {
        Self {
            paused: false,
            running: false,
            ticks: 0,
        }
    }

    pub fn cycle(&mut self, _cycles: i32) {}
}