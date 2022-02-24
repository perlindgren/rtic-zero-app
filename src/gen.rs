use cortex_m_semihosting::{debug, hprintln};

use crate::app;

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    hprintln!("hello").ok();

    app::hello();

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
