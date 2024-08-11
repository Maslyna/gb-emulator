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
macro_rules! between {
    ($a:expr, $b:expr, $c:expr) => {
        ($a >= $b) && ($a <= $c)
    };
}

#[macro_export]
macro_rules! reverse_u16 {
    ($n:expr) => {
        (($n & 0xFF00) >> 8) | (($n & 0x00FF) << 8)
    };
}

#[macro_export]
macro_rules! combine_bytes {
    ($lo:expr, $hi:expr) => {
        ($lo as u16) | (($hi as u16) << 8)
    };
}
