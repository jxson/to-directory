use std;
use bincode;

error_chain! {
    errors {
        UnknownHomeDirectory
        FailedToDeriveName
        BookmarkNotFound
    }

    foreign_links {
        IOError(std::io::Error);
        BincodeError(std::boxed::Box<bincode::ErrorKind>);
    }
}
