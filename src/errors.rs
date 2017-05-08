use std;
use bincode;
use std::path::PathBuf;


static ISSUE_TEMPLATE: &'static str = r#"

If you believe this error to be a bug please file an issue:

https://github.com/jxson/to-directory/issues

Include the following information:

* Operating system
* Output from to --version

"#;

error_chain! {
    errors {
        DBOpenError(path: PathBuf) {
            description("Failed to open bookmark DB.")
            display("Failed to open db file: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        ResolveError(path: PathBuf) {
            description("Failed to resolve path.")
            display("Failed to resolve {:?}.{}", path, ISSUE_TEMPLATE)
        }

        BadConfigDirectory {
            description("Unable to derive config")
            display("{}", ISSUE_TEMPLATE)
        }

        FailedToDeriveBasename(path: PathBuf) {
            description("Failed to derive basename")
            display("Could not derive basename from {:?}.{}", path, ISSUE_TEMPLATE)
        }

        BookmarkNotFound(name: String) {
            description("Bookmark not found")
            display("There is no entry for the bookmark {}", name)
        }

        DBCloseError(path: PathBuf) {
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
        IOError(std::io::Error) #[doc = "Error during IO"];
        BincodeError(std::boxed::Box<bincode::ErrorKind>);
    }
}
