use bincode;
use std;
use std::path::PathBuf;

static ISSUE_TEMPLATE: &'static str = r#"
=> If this is a bug please file an issue at: https://git.io/v96U6"#;

error_chain! {
    errors {
        BookmarkNotFound(name: String) {
            description("Bookmark not found.")
            display("The bookmark \"{}\" was not found", name)
        }

        DBOpenError(path: PathBuf) {
            description("Failed to open database.")
            display("Failed to open db file: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        DBCloseError(path: PathBuf) {
            description("Failed to close DB file")
            display("Could not close: \n\"{:?}\"\n\n{}", path, ISSUE_TEMPLATE)
        }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bookmark_not_found() {
        let name = String::from("nope");
        let err = ErrorKind::BookmarkNotFound(name);
        assert_eq!(err.description(), "Bookmark not found.");
        assert_eq!(err.to_string(), "The bookmark \"nope\" was not found");
    }

    #[test]
    fn database_open_failure() {
        let path = PathBuf::from("nope");
        let err = ErrorKind::DBOpenError(path);
        assert_eq!(err.description(), "Failed to open database.");
        assert_eq!(err.to_string(), format!("Failed to open db file: {:?}.{}", PathBuf::from("nope"), ISSUE_TEMPLATE));
    }
}
