use slog;
use slog_json;
use std;
use slog::{Drain, LevelFilter, Level};
use std::sync::Mutex;

pub fn root(verbose: bool) -> slog::Logger {
    let map = o!(
        "name" => "to",
        "ms" => slog::PushFnValue(move |_ : &slog::Record, ser| {
            let ms = ::now();
            ser.serialize(ms)
        }),
        "level" => slog::FnValue(move |record: &slog::Record| {
            record.level().as_short_str()
        }),
        "msg" => slog::PushFnValue(move |record : &slog::Record, ser| {
            ser.serialize(record.msg())
        }),
    );

    let stderr = std::io::stderr();
    let stream = slog_json::Json::new(stderr).add_key_value(map).build();
    let filter = match verbose {
        true => LevelFilter::new(stream, Level::Info),
        false => LevelFilter::new(stream, Level::Error),
    };

    let mutex = Mutex::new(filter).map(slog::Fuse);
    let log = slog::Logger::root(mutex, o!(
        "version" => env!("CARGO_PKG_VERSION")
    ));

    log
}
