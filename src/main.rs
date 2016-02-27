#[macro_use]
extern crate clap;

fn main() {
    let version = crate_version!();
    let matches = clap::App::new("to")
                      .about("Bookmark directories")
                      .version(version)
                      .author("Jason Campbell <jason@artifact.sh>")
                      .get_matches();
}
