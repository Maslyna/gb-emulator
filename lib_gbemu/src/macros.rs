#[macro_export]
macro_rules! bit {
    ($a:expr, $n:expr) => {
        ($a & (1 << $n)) != 0
    };
}

/// Creates 2nd mutable reference from a mutable reference
#[macro_export]
macro_rules! make_mut_ref {
    ($value:expr) => {
        unsafe {
            &mut *($value as *const _ as *const u8 as *mut u8 as *mut _)
        }
    };
}

/// Creates 2nd reference from ANY reference (mutable/immutable)
#[macro_export]
macro_rules! make_ref {
    ($value:expr) => {
        unsafe {
            &*($value as *const _ as *const u8 as *const u8 as *const _)
        }
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

