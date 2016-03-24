#[macro_use]
extern crate quick_error;

use std::{io, string, fs};
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

quick_error! {
    #[derive(Debug)]
    pub enum FileError {
        Io(e: io::Error) {
            from()
        }
        Utf8(e: string::FromUtf8Error) {
            from()
        }
    }
}

/// Reads the file at `path` and returns its contents.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, FileError> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));

    Ok(s)
}

/// Writes `contents` to a file at `path`.
///
/// If the file doesn't exist, creates it.
/// Otherwise, **truncates** the file and overwrites its previous contents, if any.
pub fn write_file<P: AsRef<Path>>(path: P, contents: &[u8]) -> Result<(), io::Error> {
    let mut file = try!(fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(path));

    try!(file.write(contents));

    Ok(())
}


// TODO: tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
