use crate::memory::Bus;
use std::os::raw::c_char;
use std::ffi::CStr;

pub struct GBDebug {
    dbg_msg: [c_char; 1024],
    msg_size: usize,
}

impl GBDebug {
    pub const fn new() -> Self {
        Self {
            dbg_msg: [0; 1024],
            msg_size: 0
        }
    }

    pub fn update(&mut self, bus: &mut Bus) {
        if bus.read(0xFF02) == 0x81 {
            let data = bus.read(0xFF01);
            self.dbg_msg[self.msg_size] = data as c_char;
            self.msg_size += 1;
            bus.write(0xFF02, 0);
        }
    }

    pub fn print(&mut self) {
        if self.dbg_msg[0] != 0 {       
            let data = unsafe {CStr::from_ptr(self.dbg_msg.as_ptr())};
            println!("Debug: {}\n", data.to_str().unwrap_or("Error in debug string"));
        }
    }
}

impl Default for GBDebug {
    fn default() -> Self {
        Self::new()
    }
}
