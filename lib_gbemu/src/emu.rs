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

    pub fn delay(milis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(milis));
    }

    pub fn get_ticks() -> u64 {
        let now = std::time::SystemTime::now();
        now.duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}
