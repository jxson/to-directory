extern crate assert_cli;

use assert_cli::Assert;

#[test]
#[ignore]
fn to_init_flag() {
    let test = Assert::main_binary()
        .with_args(&["--init"])
        .succeeds()
        .and()
        .stdout()
        .contains(include_str!("../src/to.sh"))
        .unwrap();
}
