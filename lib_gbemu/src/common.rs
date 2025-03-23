use std::{
    fs::OpenOptions,
    io::Write,
    sync::mpsc::{self, Sender},
    thread,
};

const DEBUG_FILE: &str = "debug_out.txt";
const DEBUG_OUTPUT: bool = false; // Change to false to disable logging

static mut LOGGER: Option<Sender<String>> = None;

pub fn init_logger() {
    let (tx, rx) = mpsc::channel::<String>();

    let _ = thread::Builder::new()
        .name("Logger".to_string())
        .spawn(move || {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(DEBUG_FILE)
                .expect("Failed to open debug file");

            while let Ok(data) = rx.recv() {
                if file.write_all(data.as_bytes()).is_err() {
                    eprintln!("Failed to write to debug file");
                }
            }
        });

    unsafe {
        LOGGER = Some(tx);
    }
}

pub fn debug_write(data: String) {
    if DEBUG_OUTPUT {
        unsafe {
            if let Some(ref tx) = LOGGER {
                let _ = tx.send(data);
            }
        }
    }
}
