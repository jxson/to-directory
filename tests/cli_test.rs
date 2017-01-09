extern crate to;

use to::cli;

#[test]
fn cli_no_flags() {
    let options = cli::run();
    assert_eq!(options.verbose, false);
}

#[test]
fn cli_flag_verbose() {
    let options = cli::_run(vec![]);
    assert_eq!(options.verbose, false);

    let options = cli::_run(vec!["--verbose"]);
    assert_eq!(options.verbose, true);

    let options = cli::_run(vec!["-v"]);
    assert_eq!(options.verbose, true);
}
