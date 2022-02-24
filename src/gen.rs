// Machine generated, DO NOT TOUCH!
use crate::app::*;

pub mod init {
    pub struct Context {}
}

pub mod idle {
    pub struct Context {}
}

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    let a = init::Context {};
    init(init::Context {});

    idle(idle::Context {});
}
