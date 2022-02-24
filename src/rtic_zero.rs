// Machine generated, DO NOT TOUCH!

pub mod init {
    pub struct Context {}
}

pub mod idle {
    pub struct Context {}
}

#[no_mangle]
unsafe extern "C" fn main() -> ! {
    init(init::Context {});

    idle(idle::Context {});
}

extern "Rust" {
    fn init(_cx: init::Context);

    fn idle(cx: idle::Context) -> !;
}
