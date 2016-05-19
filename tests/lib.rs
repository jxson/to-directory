use std::env;
use std::process::Command;

#[derive(Debug)]
pub struct NiceOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

fn run(args: Vec<&str>) -> NiceOutput {
    let mut directory = env::current_exe().unwrap();
            directory.pop(); // chop off exe name but leave "debug"
    let name = format!("to{}", env::consts::EXE_SUFFIX);
    let binary = directory.join(&name);

    let mut command = Command::new(binary);
            command.args(&args);
    let output = command.output().unwrap();

    return NiceOutput {
        status: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    };
}

#[test]
fn smoke_test() {
    let result = run(vec!["-h"]);
    assert_eq!(result.status, 0);
}
