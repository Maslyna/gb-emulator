#[derive(Default, Debug, Clone, Copy)]
pub struct GamepadState {
    pub start: bool,
    pub select: bool,
    pub a: bool,
    pub b: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selector {
    Button,
    Direction,
}

#[derive(Debug)]
pub struct Gamepad {
    pub state: GamepadState,
    selector: Selector,
}


impl Gamepad {
    pub fn new() -> Self {
        Self {
            selector: Selector::Button,
            state: GamepadState::default(),
        }
    }

    pub fn set_selector(&mut self, value: u8) {
        self.selector = match (value & 0x20 != 0, value & 0x10 != 0) {
            (true, _) => Selector::Button,
            (_, true) => Selector::Direction,
            _ => self.selector,
        };
    }

    pub fn set_state(&mut self, state: GamepadState) {
        self.state = state;
    }

    pub fn get_state(&self) -> &GamepadState {
        &self.state
    }

    pub fn get_state_mut(&mut self) -> &mut GamepadState {
        &mut self.state
    }

    pub fn calculate_output(&self) -> u8 {
        let mut output = 0xCF;

        match self.selector {
            Selector::Button => {
                if self.state.start {
                    output &= !(1 << 3);
                }
                if self.state.select {
                    output &= !(1 << 2);
                }
                if self.state.a {
                    output &= !(1 << 0);
                }
                if self.state.b {
                    output &= !(1 << 1);
                }
            }
            Selector::Direction => {
                if self.state.left {
                    output &= !(1 << 1);
                }
                if self.state.right {
                    output &= !(1 << 0);
                }
                if self.state.up {
                    output &= !(1 << 2);
                }
                if self.state.down {
                    output &= !(1 << 3);
                }
            }
        }

        output
    }
}

impl Default for Gamepad {
    fn default() -> Self {
        Self::new()
    }
}
