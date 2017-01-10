extern crate to;

use to::cli;
use to::cli::Action;

#[test]
fn cli_no_flags() {
    let options = cli::run();
    assert_eq!(options.verbose, false);
}

#[test]
fn cli_name() {
    let options = cli::_run(vec!["foo"]);
    assert_eq!(options.action, Action::None);
    assert_eq!(options.name, Some(String::from("foo")));
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

#[test]
fn cli_flag_info() {
    let options = cli::_run(vec!["--info"]);
    assert_eq!(options.action, Action::GetBookmark);

    let options = cli::_run(vec!["-i"]);
    assert_eq!(options.action, Action::GetBookmark);
}

#[test]
fn cli_flag_save() {
    let options = cli::_run(vec!["--save"]);
    assert_eq!(options.action, Action::PutBookmark);

    let options = cli::_run(vec!["-s"]);
    assert_eq!(options.action, Action::PutBookmark);
}

#[test]
fn cli_flag_list() {
    let options = cli::_run(vec!["--list"]);
    assert_eq!(options.action, Action::ListBookmarks);

    let options = cli::_run(vec!["-l"]);
    assert_eq!(options.action, Action::ListBookmarks);
}

#[test]
fn cli_flag_delete() {
    let options = cli::_run(vec!["foo", "--delete"]);
    assert_eq!(options.action, Action::DeleteBookmark);

    let options = cli::_run(vec!["foo", "-d"]);
    assert_eq!(options.action, Action::DeleteBookmark);
}

#[test]
fn cli_flag_init() {
    let options = cli::_run(vec!["--init"]);
    assert_eq!(options.initialize, true);
}
