extern crate assert_cli;
extern crate tap_rust;
extern crate glob;
extern crate bincode;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use bincode;
use glob::glob;
// use assert_cli::Assert;
// use tap_rust::tap_writer::TapWriter;
use std::env;
use std::path::PathBuf;
use std::fs::File;

// https://stackoverflow.com/questions/35711044/specify-binary-only-dependencies
// https://github.com/Cigna/TAP-Rust
// https://github.com/killercup/assert_cli/

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = format!("{}/tests/integration/*.json", dir);

    for entry in glob(&path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => test(&path),
            Err(e) => println!("{:?}", e),
        }
    }

    // let test = Assert::main_binary()
    //     .with_args(&["--init"])
    //     .succeeds()
    //     .and()
    //     .stdout()
    //     .contains(include_str!("../src/to.sh"))
    //     .execute();
    //
    // let writer = TapWriter::new("Example TAP stream");
    //
    // writer.plan(1, 1);
    //
    // writer.name();
    //
    // match test {
    //     Ok(_) => writer.ok(1, "--init"),
    //     Err(err) => {
    //         writer.not_ok(1, "--init");
    //         let message = format!("{}", err);
    //         writer.diagnostic(message.as_str());
    //     },
    // }
}

error_chain!{
    foreign_links {
        IOError(std::io::Error) #[doc = "Error during IO"];
        SerdeError(serde_json::Error);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Spec {
    pub name: String,
    pub status: u32,
}

fn test(path: &PathBuf) {
    let spec = hydrate(path);
    println!("{:?}", spec);
}

fn hydrate(path: &PathBuf) -> Result<Spec> {
    let file = try!(File::open(path));
    let spec = try!(serde_json::from_reader(file));
    Ok(spec)
}
