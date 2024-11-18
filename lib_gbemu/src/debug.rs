use crate::memory::Bus;

pub struct GBDebug {
    dbg_msg: Vec<u8>,
}

impl GBDebug {
    pub fn new() -> Self {
        Self {
            dbg_msg: Vec::new(),
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        if bus.read(0xFF02) == 0x81 {
            let data = bus.read(0xFF01);
            self.dbg_msg.push(data);
            bus.write(0xFF02, 0);
        }
    }

    pub fn print(&mut self) {
        if !self.dbg_msg.is_empty() && self.dbg_msg[0] != 0 {
            let data = String::from_utf8_lossy(&self.dbg_msg);
            println!("Debug: {}\n", data);
        }
    }
}

impl Default for GBDebug {
    fn default() -> Self {
        Self::new()
    }
}
