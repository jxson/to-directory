extern crate to;
extern crate tempdir;

use to::dir;
use tempdir::TempDir;

#[test]
fn dir_mkdirp() {
    let tmp = TempDir::new("test").unwrap();
    let path = tmp.path().join("does-not-exist");

    assert_eq!(path.exists(), false);
    assert_eq!(dir::mkdirp(&path).is_ok(), true);
    assert_eq!(path.exists(), true);
    assert_eq!(dir::mkdirp(&path).is_ok(), true);
}
