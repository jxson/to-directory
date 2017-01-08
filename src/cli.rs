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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let options = match run() {
            Ok(options) => options,
            Err(err) => panic!(err),
        };
        assert_eq!(options.verbose, false);
    }
}
