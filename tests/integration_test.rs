mod cli;

#[test]
fn cli_help() {
    let out = cli::run(vec!["--help"]).expect("--help failed");
    assert_eq!(out.status, 0);
    assert!(out.stdout.len() > 0);
    assert!(out.stdout.contains("USAGE"));
    assert!(out.stderr.is_empty());
}
