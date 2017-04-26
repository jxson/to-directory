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
        BadConfigDirectory {
            description("Unable to derive config")
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

        FailedToOpenDatabase(path: PathBuf) {
            description("Failed to open DB file")
            display("Could not open: \n\"{:?}\"\n\n{}", path, ISSUE_TEMPLATE)
        }

        FailedToCloseDatabase(path: PathBuf) {
            description("Failed to close DB file")
            display("Could not close: \n\"{:?}\"\n\n{}", path, ISSUE_TEMPLATE)
        }

        InfoFlagRequiresName {
            description("--info requires <name>")
            display("For example: to -i foo")
        }

        DeleteFlagRequiresName {
            description("--delete requires <name>")
            display("For example: to -d foo")
        }

        ToRequiresName {
            description("requires <name>")
            display("For example: to foo")
        }
    }

    foreign_links {
        IOError(std::io::Error);
        BincodeError(std::boxed::Box<bincode::ErrorKind>);
    }
}
