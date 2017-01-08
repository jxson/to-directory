use errors::*;

pub fn run() -> Result<Options> {
    let options = Options::new();
    return Ok(options);
}

#[derive(Debug)]
pub struct Options {
    pub verbose: bool,
}

impl Options {
    fn new() -> Options {
        return Options {
            verbose: false,
        };
    }
}
