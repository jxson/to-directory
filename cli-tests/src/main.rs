#[macro_use] extern crate serde_derive;
extern crate failure;
extern crate glob;
extern crate serde_json;
extern crate serde;
extern crate tape;

use failure::Error;
use glob::glob;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::fs::File;
use tape::test;

fn main() {
    if let Err(err) = run() {
        println!("{}, {}", err.cause(), err.backtrace());
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let start = env::var("CARGO_MANIFEST_DIR")?;
    let pattern = format!("{}/../tests/integration/*.json", start);

    for entry in glob(&pattern)? {
        let dir = entry?;
        let path = PathBuf::from(dir);
        let spec = Spec::from(&path)?;

        test(spec.name, |t| {
            t.assert(true);
        });

        // decode json entry into a struct.
        // run each struct through a manual test
        // each test should look similar to https://github.com/substack/tape

    }

    Ok(())
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Spec {
    pub name: String,
    pub status: u8,
    pub args: Vec<String>,
}

impl Spec {
    fn from(path: &PathBuf) -> Result<Spec, Error> {
        let file = File::open(path)?;
        let spec = serde_json::from_reader(file)?;
        Ok(spec)
    }
}
