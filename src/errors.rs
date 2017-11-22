use log;
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
            description("Failed to close database.")
            display("Failed to close db file: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        ResolveError(path: PathBuf) {
            description("Failed to resolve path.")
            display("Failed to resolve: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        PathDoesNotExistError(path: PathBuf) {
            description("Path does not exist.")
            display("Path does not exist: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        CurrentDirectoryError(path: PathBuf) {
            description("Failed to derive current directory.")
            display("Failed to get current dir when resolving: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        ConfigError {
            description("Unable to derive config")
            display("{}", ISSUE_TEMPLATE)
        }

        BasenameError(path: PathBuf) {
            description("Failed to derive basename")
            display("Could not derive basename from: {:?}.{}", path, ISSUE_TEMPLATE)
        }

        CreateDirError(path: PathBuf) {
            description("Failed to create dir")
            display("Failed to create {:?}.{}", path, ISSUE_TEMPLATE)
        }
    }

    foreign_links {
        IOError(std::io::Error) #[doc = "Error during IO"];
        BincodeError(std::boxed::Box<bincode::ErrorKind>);
        SetLoggerError(log::SetLoggerError);
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
        assert_eq!(
            err.to_string(),
            format!(
                "Failed to open db file: {:?}.{}",
                PathBuf::from("nope"),
                ISSUE_TEMPLATE
            )
        );
    }

    #[test]
    fn database_close_failure() {
        let path = PathBuf::from("nope");
        let err = ErrorKind::DBCloseError(path);
        assert_eq!(err.description(), "Failed to close database.");
        assert_eq!(
            err.to_string(),
            format!(
                "Failed to close db file: {:?}.{}",
                PathBuf::from("nope"),
                ISSUE_TEMPLATE
            )
        );
    }


    #[test]
    fn path_resolve_error() {
        let path = PathBuf::from("nope");
        let err = ErrorKind::ResolveError(path);
        assert_eq!(err.description(), "Failed to resolve path.");
        assert_eq!(
            err.to_string(),
            format!(
                "Failed to resolve: {:?}.{}",
                PathBuf::from("nope"),
                ISSUE_TEMPLATE
            )
        );
    }

    #[test]
    fn path_does_not_exist_error() {
        let path = PathBuf::from("nope");
        let err = ErrorKind::PathDoesNotExistError(path);
        assert_eq!(err.description(), "Path does not exist.");
        assert_eq!(
            err.to_string(),
            format!(
                "Path does not exist: {:?}.{}",
                PathBuf::from("nope"),
                ISSUE_TEMPLATE
            )
        );
    }

    #[test]
    fn cwd_error() {
        let path = PathBuf::from("nope");
        let err = ErrorKind::CurrentDirectoryError(path);
        assert_eq!(err.description(), "Failed to derive current directory.");
        assert_eq!(
            err.to_string(),
            format!(
                "Failed to get current dir when resolving: {:?}.{}",
                PathBuf::from("nope"),
                ISSUE_TEMPLATE
            )
        );
    }

    #[test]
    fn config_error() {
        let err = ErrorKind::ConfigError;
        assert_eq!(err.description(), "Unable to derive config");
        assert_eq!(err.to_string(), format!("{}", ISSUE_TEMPLATE));
    }

    #[test]
    fn basename_error() {
        let path = PathBuf::from("nope");
        let err = ErrorKind::BasenameError(path);
        assert_eq!(err.description(), "Failed to derive basename");
        assert_eq!(
            err.to_string(),
            format!(
                "Could not derive basename from: {:?}.{}",
                PathBuf::from("nope"),
                ISSUE_TEMPLATE
            )
        );
    }

    #[test]
    fn create_dir_error() {
        let path = PathBuf::from("nope");
        let err = ErrorKind::CreateDirError(path);
        assert_eq!(err.description(), "Failed to create dir");
        assert_eq!(
            err.to_string(),
            format!(
                "Failed to create {:?}.{}",
                PathBuf::from("nope"),
                ISSUE_TEMPLATE
            )
        );
    }
}
