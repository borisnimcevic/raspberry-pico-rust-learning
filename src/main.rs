#![no_std]
#![no_main]

use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

#[entry]
fn main() -> ! {
    // some setup

    loop {
        // toggle LED (GPIO 25)
    }
}
