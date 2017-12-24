extern crate assert_cli;

use assert_cli::Assert;

fn main() {
    let test = Assert::main_binary()
        .with_args(&["--init"])
        .succeeds()
        .and()
        .stdout()
        .contains(include_str!("../src/to.sh"))
        .unwrap();
}
