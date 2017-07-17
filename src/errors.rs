use std;
use bincode;
use std::path::PathBuf;


static ISSUE_TEMPLATE: &'static str = r#"
=> If this is a bug please file an issue at: https://git.io/v96U6"#;

error_chain! {
    errors {
        // CLI errors.
        BookmarkNotFound(name: String) {
            description("Bookmark not found")
            display("There is no entry for the bookmark {}", name)
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

        // Bookmark DB errors.
        DBOpenError(path: PathBuf) {
            description("Failed to open bookmark DB.")
            display("Failed to open db file: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        DBCloseError(path: PathBuf) {
            description("Failed to close DB file")
            display("Could not close: \n\"{:?}\"\n\n{}", path, ISSUE_TEMPLATE)
        }

        // Directory and path errors.
        ResolveError(path: PathBuf) {
            description("Failed to resolve path.")
            display("Failed to resolve {:?}.{}", path, ISSUE_TEMPLATE)
        }

        PathDoesNotExistError(path: PathBuf) {
            description("Path does not exist.")
            display("Path does not exist {:?}.{}", path, ISSUE_TEMPLATE)
        }

        CurrentDirectoryError(path: PathBuf) {
            description("Failed to derive current directory.")
            display("Failed to get current dir when resolving {:?}.{}", path, ISSUE_TEMPLATE)
        }

        ConfigError {
            description("Unable to derive config")
            display("{}", ISSUE_TEMPLATE)
        }

        BasenameError(path: PathBuf) {
            description("Failed to derive basename")
            display("Could not derive basename from {:?}.{}", path, ISSUE_TEMPLATE)
        }

        CreateDirError(path: PathBuf) {
            description("Failed to create dir")
            display("Failed to create {:?}.{}", path, ISSUE_TEMPLATE)
        }
    }

    foreign_links {
        IOError(std::io::Error) #[doc = "Error during IO"];
        BincodeError(std::boxed::Box<bincode::ErrorKind>);
    }
}
