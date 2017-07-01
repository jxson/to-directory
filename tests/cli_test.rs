extern crate to;

use std::env;
use std::path::PathBuf;
use to::cli::Action;
use to::cli;

fn run(mut args: Vec<&str>) -> cli::Options {
    args.insert(0, "to");
    let matches = cli::app().get_matches_from(args);
    cli::Options::new(matches)
}

#[test]
fn cli_defaults() {
    let options = run(vec![]);
    assert_eq!(options.verbose, false);
    assert_eq!(options.initialize, false);
    assert_eq!(options.name, None);
    assert_eq!(options.action, Action::Pathname);
}

#[test]
fn cli_name() {
    let options = run(vec!["foo"]);
    assert_eq!(options.action, Action::Pathname);
    assert_eq!(options.name, Some(String::from("foo")));

    let options = run(vec!["Foo"]);
    assert_eq!(options.action, Action::Pathname);
    assert_eq!(options.name, Some(String::from("foo")));

    let options = run(vec!["foo/"]);
    assert_eq!(options.name, Some(String::from("foo")));
}

#[test]
fn cli_path() {
    let options = run(vec!["project", "~/code/project"]);
    assert_eq!(options.path, Some(PathBuf::from("~/code/project")));
}

#[test]
fn cli_flag_verbose() {
    assert_eq!(run(vec![]).verbose, false);

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
fn cli_flag_config_default() {
    let options = run(vec![]);
    let expected = env::home_dir().map(|mut home| {
        home.push(".to");
        home
    });

    assert_eq!(options.config, expected);
}

#[test]
fn cli_flag_config() {
    let options = run(vec!["--config", "~/whatever"]);
    let expected = PathBuf::from("~/whatever");

    assert_eq!(options.config, Some(expected));
}
