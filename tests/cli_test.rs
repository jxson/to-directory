extern crate to;
extern crate tempdir;

use tempdir::TempDir;
use to::cli;
use to::cli::Action;

fn run(mut args: Vec<&str>) -> cli::Options {
    args.insert(0, "to");
    let matches = cli::app().get_matches_from(args);
    cli::Options::new(matches)
}

#[test]
fn cli_name() {
    let options = run(vec!["foo"]);
    assert_eq!(options.action, Action::Pathname);
    assert_eq!(options.name, Some(String::from("foo")));
}

#[test]
fn cli_name_trailing_slash() {
    let options = run(vec!["foo/"]);
    assert_eq!(options.name, Some(String::from("foo")));
}

#[test]
fn cli_name_case_insensitive() {
    let options = run(vec!["Foo"]);
    assert_eq!(options.action, Action::Pathname);
    assert_eq!(options.name, Some(String::from("foo")));
}

#[test]
fn cli_flag_none() {
    let options = run(vec![]);
    assert_eq!(options.verbose, false);
    assert_eq!(options.initialize, false);
    assert_eq!(options.name, None);
    assert_eq!(options.action, Action::Pathname);
}

#[test]
fn cli_flag_verbose() {
    let options = run(vec![]);
    assert_eq!(options.verbose, false);

    let options = run(vec!["--verbose"]);
    assert_eq!(options.verbose, true);

    let options = run(vec!["-v"]);
    assert_eq!(options.verbose, true);
}

#[test]
fn cli_flag_info() {
    let options = run(vec!["--info"]);
    assert_eq!(options.action, Action::Info);

    let options = run(vec!["-i"]);
    assert_eq!(options.action, Action::Info);
}

#[test]
fn cli_flag_save() {
    let options = run(vec!["--save"]);
    assert_eq!(options.action, Action::Save);

    let options = run(vec!["-s"]);
    assert_eq!(options.action, Action::Save);
}

#[test]
fn cli_flag_list() {
    let options = run(vec!["--list"]);
    assert_eq!(options.action, Action::List);

    let options = run(vec!["-l"]);
    assert_eq!(options.action, Action::List);
}

#[test]
fn cli_flag_delete() {
    let options = run(vec!["foo", "--delete"]);
    assert_eq!(options.action, Action::Delete);

    let options = run(vec!["foo", "-d"]);
    assert_eq!(options.action, Action::Delete);
}

#[test]
fn cli_flag_init() {
    let options = run(vec!["--init"]);
    assert_eq!(options.initialize, true);
}

#[test]
fn cli_flag_config() {
    let dir = TempDir::new("config").unwrap();
    let config_dir = dir.path().to_str().unwrap();
    let options = run(vec!["--config", config_dir]);

    assert_eq!(options.config().unwrap(), dir.path());
}
