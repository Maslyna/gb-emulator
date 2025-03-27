#[derive(Debug)]
pub struct Emu {
    pub die: bool,
    pub paused: bool,
    pub running: bool,
}

impl Emu {
    pub const fn new() -> Self {
        Self {
            die: false,
            paused: false,
            running: false,
        }
    }
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}
