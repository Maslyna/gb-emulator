#[macro_export]
macro_rules! bit {
    ($a:expr, $n:expr) => {
        ($a & (1 << $n)) != 0
    };
}

#[macro_export]
macro_rules! set_bit {
    ($a:expr, $n:expr, $on:expr) => {
        if $on {
            $a |= 1 << $n;
        } else {
            $a &= !(1 << $n);
        }
    };
}

#[macro_export]
macro_rules! bytes_to_word {
    ($lo:expr, $hi:expr) => {
        ($lo as u16) | (($hi as u16) << 8)
    };
}

macro_rules! debug {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

