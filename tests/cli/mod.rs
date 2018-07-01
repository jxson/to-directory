use std::io;
use std::process;

pub struct Output {
  pub status: i32,
  stdout: Vec<u8>,
  stderr: Vec<u8>,
}

impl Output {
  fn new(o: process::Output) -> Output {
    Output {
      status: o.status.code().expect("failed to get status code"),
      stdout: o.stdout,
      stderr: o.stderr,
    }
  }

  pub fn stdout(&self) -> String {
    let bytes = self.stdout.clone();
    String::from_utf8(bytes).expect("failed to convert stdout")
  }

  pub fn stderr(&self) -> String {
    let bytes = self.stderr.clone();
    String::from_utf8(bytes).expect("failed to convert stderr")
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
