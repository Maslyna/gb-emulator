use std::fs::OpenOptions;
use std::io::Write;

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
