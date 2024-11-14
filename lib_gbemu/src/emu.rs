#[derive(Debug)]
pub struct Emu {
    pub die: bool,
    pub paused: bool,
    pub running: bool,
    pub ticks: u64,
}

impl Emu {
    pub const fn new() -> Self {
        Self {
            die: false,
            paused: false,
            running: false,
            ticks: 0,
        }
    }

    pub fn cycle(&mut self, _cycles: i32) {}
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}