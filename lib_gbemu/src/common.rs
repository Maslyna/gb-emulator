use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
pub fn reverse(n: u16) -> u16 {
    ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
}

const DEBUG_FILE: &str = "debug_out.txt";

pub fn debug_write(data: &str) {
    let file = OpenOptions::new()
        .create(true) // Создает файл, если его нет
        .append(true) // Добавляет данные в конец файла
        .open(DEBUG_FILE);

    if let Ok(mut f) = file {
        write!(f, "{}", data).unwrap();
    }
}
