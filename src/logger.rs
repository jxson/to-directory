use slog;
use slog_json;
use chrono;
use std;
use slog::Drain;
use std::sync::Mutex;

pub fn root() -> slog::Logger {
    let out = std::io::stderr();
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
    let foo = slog_json::Json::new(out).add_key_value(map).build();
    let mutex = Mutex::new(foo).map(slog::Fuse);
    let log = slog::Logger::root(mutex, o!(
        "version" => env!("CARGO_PKG_VERSION")
    ));

    log
}
