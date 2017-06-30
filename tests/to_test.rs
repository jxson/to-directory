extern crate assert_cli;

use assert_cli::Assert;


#[test]
fn to_init_flag() {
    let test = Assert::main_binary()
        .with_args(&["--init"])
        .succeeds()
        .and()
        .prints(include_str!("../src/to.sh"))
        .execute();
    assert!(test.is_ok());
}
