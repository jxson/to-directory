extern crate log;
extern crate env_logger;

use env_logger::LogBuilder;
use error::{ToResult};
use log::{LogRecord, LogLevelFilter};

pub fn init(verbose: bool) -> ToResult<()> {
    if verbose {
        try!(init_verbose());
    } else {
        try!(init_env_logger());
    }

    return Ok(());
}

fn init_verbose() -> ToResult<()> {
    let format = |record: &LogRecord| {
        format!("to: {} - {}", record.level(), record.args())
    };

    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, LogLevelFilter::Info);

    try!(builder.init());

    return Ok(());
}

fn init_env_logger() -> ToResult<()> {
    try!(env_logger::init());

    return Ok(());
}
