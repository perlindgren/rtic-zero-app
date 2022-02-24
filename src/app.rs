// user application

use lm3s6965 as _;
use panic_halt as _;

use cortex_m_semihosting::hprintln;

pub fn hello() {
    hprintln!("my hello").ok();
}
