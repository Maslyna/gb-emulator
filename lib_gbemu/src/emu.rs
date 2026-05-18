// Copyright 2026 Maslyna AKA Mykhailo Ordyntsev
// SPDX-License-Identifier: gpl-3.0-only

#[derive(Debug)]
pub struct Emu {
    pub die: bool,
    pub paused: bool,
    pub running: bool,
}

impl Emu {
    pub const fn new() -> Self {
        Self {
            die: false,
            paused: false,
            running: false,
        }
    }
}

impl Default for Emu {
    fn default() -> Self {
        Self::new()
    }
}
