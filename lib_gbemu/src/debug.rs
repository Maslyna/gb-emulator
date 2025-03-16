use crate::memory::Bus;

pub struct GsSerial {
    dbg_msg: Vec<char>,
}

impl GsSerial {
    pub fn new() -> Self {
        Self {
            dbg_msg: Vec::new(),
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        if bus.read(0xFF02) == 0x81 {
            let data = bus.read(0xFF01) as char;
            self.dbg_msg.push(data);
            bus.write(0xFF02, 0);
        }
    }

    pub fn print(&mut self) {
        if !self.dbg_msg.is_empty() && self.dbg_msg[0] != ' ' {
            let dbg_str: String = self.dbg_msg[..self.dbg_msg.len()].iter().collect();
            println!("DBG {}", dbg_str)
        }
    }
}

impl Default for GsSerial {
    fn default() -> Self {
        Self::new()
    }
}
