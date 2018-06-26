extern crate assert_cli;

use assert_cli::Assert;

fn main() {
  Assert::main_binary()
    .with_args(&["--help"])
    .succeeds()
    .unwrap();

  Assert::main_binary()
    .with_args(&["--init"])
    .succeeds()
    .and()
    .stdout()
    .contains(include_str!("../src/to.sh"))
    .unwrap();
}
