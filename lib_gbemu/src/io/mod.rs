pub mod timer;

static mut SERIAL_DATA: [u8; 2] = [0, 0];

pub fn read(address: u16) -> u8 {
    if address == 0xFF01 {
        unsafe { return SERIAL_DATA[0] };
    }

    if address == 0xFF02 {
        unsafe { return SERIAL_DATA[1] };
    }

    println!("UNSUPPORTED BUS READ {:04X}", address);

    0
}

pub fn write(address: u16, value: u8) {
    if address == 0xFF01 {
        unsafe { SERIAL_DATA[0] = value };
        return;
    }

    if address == 0xFF02 {
        unsafe { SERIAL_DATA[1] = value };
        return;
    }

    println!("UNSUPPORTED BUS WRITE {:04X} VALUE {:04X}", address, value);
}
