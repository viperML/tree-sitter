use std::fs::ReadDir;
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

pub struct PathResults {
    bases: Vec<PathBuf>,
    filename: String,
    reader: Option<ReadDir>,
}

#[must_use]
pub fn in_path<P, S>(base: &[P], filename: S) -> PathResults
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let bases = base.iter().map(|p| p.as_ref().to_owned()).collect();

    PathResults {
        bases,
        filename: filename.as_ref().to_owned(),
        reader: None,
    }
}

impl Iterator for PathResults {
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reader.is_none() && !self.bases.is_empty() {
            let next_base = self.bases.pop().unwrap();
            self.reader = match next_base.read_dir() {
                Ok(r) => Some(r),
                Err(e) => return Some(Err(e)),
            }
        }

        if self.reader.is_none() && self.bases.is_empty() {
            return None;
        }

        let reader = self.reader.as_mut().unwrap();

        let next = reader.next();

        todo!()
    }
}
