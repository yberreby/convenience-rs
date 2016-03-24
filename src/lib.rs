#[macro_use]
extern crate quick_error;

use std::{io, string};
use std::fs::File;
use std::path::Path;
use std::io::Read;

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

/// Read the file at `path` and returns its contents.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, FileError> {
    let mut s = String::new();
    let mut f = try!(File::open(path));
    try!(f.read_to_string(&mut s));

    Ok(s)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
