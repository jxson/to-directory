extern crate to;

use to::cli;
use to::cli::Action;

#[test]
fn cli_name() {
    let options = cli::from(vec!["foo"]);
    assert_eq!(options.action, Action::None);
    assert_eq!(options.name, Some(String::from("foo")));
}

#[test]
fn cli_flag_none() {
    let options = cli::run();
    assert_eq!(options.verbose, false);
    assert_eq!(options.initialize, false);
    assert_eq!(options.name, None);
    assert_eq!(options.action, Action::None);
}

#[test]
fn cli_flag_verbose() {
    let options = cli::from(vec![]);
    assert_eq!(options.verbose, false);

    let options = cli::from(vec!["--verbose"]);
    assert_eq!(options.verbose, true);

    let options = cli::from(vec!["-v"]);
    assert_eq!(options.verbose, true);
}

#[test]
fn cli_flag_info() {
    let options = cli::from(vec!["--info"]);
    assert_eq!(options.action, Action::GetBookmark);

    let options = cli::from(vec!["-i"]);
    assert_eq!(options.action, Action::GetBookmark);
}

#[test]
fn cli_flag_save() {
    let options = cli::from(vec!["--save"]);
    assert_eq!(options.action, Action::PutBookmark);

    let options = cli::from(vec!["-s"]);
    assert_eq!(options.action, Action::PutBookmark);
}

#[test]
fn cli_flag_list() {
    let options = cli::from(vec!["--list"]);
    assert_eq!(options.action, Action::ListBookmarks);

    let options = cli::from(vec!["-l"]);
    assert_eq!(options.action, Action::ListBookmarks);
}

#[test]
fn cli_flag_delete() {
    let options = cli::from(vec!["foo", "--delete"]);
    assert_eq!(options.action, Action::DeleteBookmark);

    let options = cli::from(vec!["foo", "-d"]);
    assert_eq!(options.action, Action::DeleteBookmark);
}

#[test]
fn cli_flag_init() {
    let options = cli::from(vec!["--init"]);
    assert_eq!(options.initialize, true);
}
