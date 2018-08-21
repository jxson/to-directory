use bincode;
use log;
use std;
use std::path::PathBuf;

static ISSUE_TEMPLATE: &'static str = r#"
=> If this is a bug please file an issue at: https://git.io/v96U6"#;

error_chain! {
    errors {

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
