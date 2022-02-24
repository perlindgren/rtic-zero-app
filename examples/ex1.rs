// Fixed, DO NOT TOUCH!
#![no_std]
#![no_main]

use rtic::rtic_zero::*;

use cortex_m_semihosting::{debug, hprintln};
use lm3s6965 as _;
use panic_halt as _;

#[no_mangle]
pub fn init(_cx: init::Context) {
    hprintln!("init").ok();
}

#[no_mangle]
pub fn idle(_cx: idle::Context) -> ! {
    hprintln!("idle").ok();
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
