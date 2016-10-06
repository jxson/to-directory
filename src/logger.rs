extern crate log;
extern crate env_logger;

use error::{ToResult};
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;

pub fn init(should_log: bool) -> ToResult<()> {
    if should_log {
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
