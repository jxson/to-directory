use std::io;
use std::process;

pub struct Output {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl Output {
    fn new(o: process::Output) -> Output {
        Output {
            status: o.status.code().expect("failed to get status code"),
            stdout: String::from_utf8(o.stdout).expect("failed to convert stdout"),
            stderr: String::from_utf8(o.stderr).expect("failed to convert stderr"),
        }
    }
}

pub fn run(mut args: Vec<&str>) -> Result<Output, io::Error> {
    args.insert(0, "run");
    args.insert(1, "--quiet");
    args.insert(2, "--");

    process::Command::new("cargo")
        .args(args)
        .output()
        .map(Output::new)
}
