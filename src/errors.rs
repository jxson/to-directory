use std;

error_chain! {
    errors {
        UnknownHomeDirectory
    }

    foreign_links {
        IOError(std::io::Error);
    }
}
