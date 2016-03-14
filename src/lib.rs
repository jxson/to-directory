#[macro_use]
extern crate bincode;
extern crate chrono;
extern crate rustc_serialize;

pub mod cli;
pub mod store;
pub mod types;

pub use types::{
    ToError,
    ToResult,
    Bookmark,
};
