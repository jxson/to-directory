static mut VERBOSE : bool = false;

pub fn init(should_log: bool) {
    unsafe { VERBOSE = should_log; }
}

pub fn __verbose() -> bool {
    return unsafe { VERBOSE };
}

macro_rules! debug {
    ($fmt:expr) => ({
        if logger::__verbose() {
            println!("debug: {}", $fmt);
        }
    });
}
