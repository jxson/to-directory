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
            ser.emit(ms)
        }),
        "level" => slog::FnValue(move |record: &slog::Record| {
            record.level().as_short_str()
        }),
        "msg" => slog::PushFnValue(move |record : &slog::Record, ser| {
            ser.emit(record.msg())
        }),
    );

    let stderr = std::io::stderr();
    let stream = slog_json::Json::new(stderr).add_key_value(map).build();

    let filter = if verbose {
        LevelFilter::new(stream, Level::Info)
    } else {
        LevelFilter::new(stream, Level::Error)
    };

    let mutex = Mutex::new(filter).map(slog::Fuse);

    slog::Logger::root(mutex,
                       o!(
        "version" => env!("CARGO_PKG_VERSION")
    ))
}
