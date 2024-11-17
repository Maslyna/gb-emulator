struct Timer {
    _divider: u16,
    _counter: u8,
    _modulo: u8,
    _timer_control: u8,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            _divider: 0xAC00,
            _counter: 0,
            _modulo: 0,
            _timer_control: 0,
        }
    }

    pub fn _tick(&mut self) {
        let _prev_divider = self._divider;
        self._divider += 1;

        let _timer_update = false;

        // match self.counter & 0b11 {
        //     0b00 => {
        //         timer_update = ((prev_divider & (1 << 9)) && (!(self.divider & (1 << 9)))) != 0;
        //     }
        // };
    }
}
