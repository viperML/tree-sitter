use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

pub fn find_in_path<P, S>(base: P, filename: S) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let base = base.as_ref();
    let filename = filename.as_ref();

    for subpath in base.read_dir()? {
        let subpath = subpath?.path();
        let res = subpath.join(filename);
        if res.exists() {
            return Ok(res);
        }
    }

    Err(io::Error::from(ErrorKind::NotFound))
}
