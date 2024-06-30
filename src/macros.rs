#![allow(unused_macros)]
macro_rules! bit {
    ($a:expr, $n:expr) => {
        (($a & (1 << $n)) != 0) as i32
    };
}

macro_rules! bit_set {
    ($a:expr, $n:expr, $on:expr) => {
        if $on {
            $a |= 1 << $n;
        } else {
            $a &= !(1 << $n);
        }
    };
}

macro_rules! between {
    ($a:expr, $b:expr, $c:expr) => {
        ($a >= $b) && ($a <= $c)
    };
}
