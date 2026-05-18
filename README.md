# gb-emulator
 
A Game Boy (DMG) emulator written in Rust.
 
## Project Structure
 
This is a Cargo workspace consisting of two crates:
 
```
gb-emulator/
├── lib_gbemu/       # Core emulation library
├── gbemu/           # Executable frontend
├── roms/            # ROM files directory
├── debug_config.txt # Debug configuration
├── rustfmt.toml     # Rust formatting config
└── test.py          # Utility test script
```
 
- **`lib_gbemu`** — the heart of the emulator: CPU, PPU, memory bus, timers, and other hardware components implemented as a reusable library.
- **`gbemu`** — the binary crate that depends on `lib_gbemu` and provides the runnable emulator frontend.
## 🚀 Getting Started
 
### Prerequisites
 
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
### Build
 
```bash
git clone https://github.com/Maslyna/gb-emulator.git
cd gb-emulator
cargo build --release
```
 
### Run
 
```bash
cargo run --release --bin gbemu -- <path-to-rom>
```
 
Place your ROM files in the `roms/` directory for convenience:
 
```bash
cargo run --release --bin gbemu -- roms/game.gb
```
 
## 🛠️ Development
 
### Building in debug mode
 
```bash
cargo build
```
 
### Running tests
 
```bash
cargo test
```
 
### Code formatting
 
The project uses a custom `rustfmt.toml` configuration. To format the code:
 
```bash
cargo fmt
```
 
### Debug configuration
 
The `debug_config.txt` file can be used to configure debug-mode behaviour of the emulator (e.g., logging verbosity, breakpoints).
 
## Tech Stack
 
| Component | Details |
|-----------|---------|
| Language  | Rust    |
| Build system | Cargo (workspace) |
| Target platform | Game Boy (DMG) |
 
## 🎲 Compatible ROMs
 
> ⚠️ **Anti-Piracy Notice**
>
> This emulator was developed and tested exclusively using **open-source homebrew ROMs** freely distributed by the community. I am strongly against software piracy. Do **not** use this emulator to play commercial Game Boy games unless you legally own the original cartridge.
 
The following are free, open-source homebrew games you can use with this emulator:

### [Blargg's Test ROMs](https://github.com/retrio/gb-test-roms)
The standard suite of hardware test ROMs by Shay Green (Blargg), widely used across the emulator development community to verify CPU instructions, timing, sound, and other hardware behaviour. Essential for validating emulator accuracy.
 
More open-source homebrew can be found at:
- [Homebrew Hub (gbdev.io)](https://hh.gbdev.io/) — an archive of hundreds of free GB/GBC games playable in the browser
- [itch.io Game Boy ROM tag](https://itch.io/games/tag-gameboy-rom) — indie homebrew releases
## 📚 Resources
 
The following references are invaluable for Game Boy emulator development:
 
- [Pan Docs](https://gbdev.io/pandocs/) — comprehensive Game Boy hardware documentation
- [Game Boy CPU Manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
- [Blargg's test ROMs](https://github.com/retrio/gb-test-roms) — standard emulator test suite
- [gbdev community](https://gbdev.io/)
## License
 
This project is licensed under the **GNU General Public License v3.0**. See [LICENSE](LICENSE) for full terms.
 
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
 
In short: you are free to use, modify, and distribute this software, but any derivative work must also be released under the GPL v3.

