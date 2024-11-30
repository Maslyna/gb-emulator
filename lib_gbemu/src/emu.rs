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
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}
