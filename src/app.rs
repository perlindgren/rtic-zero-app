// user application

use cortex_m_semihosting::{debug, hprintln};
use lm3s6965 as _;
use panic_halt as _;

use crate::gen::*;

pub fn init(_cx: init::Context) {
    hprintln!("init").ok();
}

pub fn idle(cx: idle::Context) -> ! {
    hprintln!("idle").ok();
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
