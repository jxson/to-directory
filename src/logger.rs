static mut VERBOSE : bool = false;

pub fn init(should_log: bool) {
    unsafe { VERBOSE = should_log; }
}

pub fn __verbose() -> bool {
    return unsafe { VERBOSE };
}

pub fn __log(string: String) {
    println!("  to => {}", string);
}

macro_rules! debug {
    ($string:expr) => ({
        if logger::__verbose() {
            logger::__log($string.to_string());
        }
    });

    ($template:expr, $($arg:tt)*) => ({
        if logger::__verbose() {
            let string = format!($template, $($arg)*);
            logger::__log(string);
        }
    });
}
