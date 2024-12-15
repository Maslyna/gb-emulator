use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
pub fn reverse(n: u16) -> u16 {
    ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
}

#[allow(dead_code)]
const DEBUG_FILE: &str = "debug_out.txt";
const DEBUG_OUTPUT: bool = false;

#[allow(dead_code)]
pub fn debug_write(data: &str) {
    if DEBUG_OUTPUT {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(DEBUG_FILE);

        if let Ok(mut f) = file {
            f.write_all(data.as_bytes()).unwrap();
        }
    }
}
