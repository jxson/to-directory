use std;
use bincode;
use std::path::PathBuf;


static ISSUE_TEMPLATE: &'static str = r#"

File an issue at https://github.com/jxson/to-directory/issues.

Include the following information:

* Operating system
* Output from to --version

"#;

error_chain! {
    errors {
        UnknownHomeDirectory {
            description("Unable to find $HOME")
            display("{}", ISSUE_TEMPLATE)
        }

        FailedToDeriveBasename(path: PathBuf) {
            description("Failed to derive basename")
            display("Could not derive basename from \n\"{:?}\"\n\n{}", path, ISSUE_TEMPLATE)
        }

        BookmarkNotFound(name: String) {
            description("Bookmark not found")
            display("There is no entry for the bookmark \"{}\"", name)
        }
    }

    foreign_links {
        IOError(std::io::Error);
        BincodeError(std::boxed::Box<bincode::ErrorKind>);
    }
}
