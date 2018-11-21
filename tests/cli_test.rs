extern crate dirs;
extern crate tempdir;
extern crate to;

use dirs::home_dir;
use std::env;
use std::path::PathBuf;
use tempdir::TempDir;
use to::cli;
use to::cli::Action;

fn run(mut args: Vec<&str>) -> cli::Options {
    args.insert(0, "to");
    let matches = cli::app().get_matches_from(args);
    cli::Options::new(matches).unwrap()
}

#[test]
fn cli_defaults() {
    let config_dir = home_dir()
        .map(|mut home| {
            home.push(".to");
            home
        })
        .unwrap();
    let current_dir = env::current_dir().unwrap();
    let options = run(vec![]);

    assert_eq!(options.verbose, false);
    assert_eq!(options.initialize, false);
    assert_eq!(options.name, to::dir::basename(&current_dir).unwrap());
    assert_eq!(options.path, current_dir);
    assert_eq!(options.action, Action::Pathname);
    assert_eq!(options.config, config_dir);
}

#[test]
fn cli_flag_init() {
    let options = run(vec!["--init"]);
    assert_eq!(options.initialize, true);
}

#[test]
fn cli_name() {
    let options = run(vec!["foo"]);
    assert_eq!(options.action, Action::Pathname);
    assert_eq!(options.name, String::from("foo"));

    let options = run(vec!["Foo"]);
    assert_eq!(options.action, Action::Pathname);
    assert_eq!(options.name, String::from("foo"));

    let options = run(vec!["foo/"]);
    assert_eq!(options.name, String::from("foo"));
}

#[test]
fn cli_path() {
    let temp = TempDir::new("test-project")
        .map(|dir| dir.into_path())
        .unwrap();
    let path = temp.canonicalize().unwrap();
    let options = run(vec!["project", path.to_str().unwrap()]);
    assert_eq!(options.path, path);
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
    let expected = home_dir()
        .map(|mut home| {
            home.push(".to");
            home
        })
        .unwrap();

    assert_eq!(options.config, expected);
}

#[test]
fn cli_flag_config() {
    let options = run(vec!["--config", "~/whatever"]);
    let expected = PathBuf::from("~/whatever");

    assert_eq!(options.config, expected);
}
