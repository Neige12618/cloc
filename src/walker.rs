use std::fs;
use std::path::PathBuf;

pub struct DirWalker {
    root: PathBuf,
}

pub struct DirWalkerIter {
    stack: Vec<PathBuf>,
}

impl DirWalker {
    pub fn new(root: PathBuf) -> Self {
        DirWalker { root }
    }

    pub fn iter(&self) -> DirWalkerIter {
        DirWalkerIter {
            stack: vec![self.root.clone()],
        }
    }
}

impl IntoIterator for DirWalker {
    type Item = PathBuf;
    type IntoIter = DirWalkerIter;

    fn into_iter(self) -> Self::IntoIter {
        DirWalkerIter {
            stack: vec![self.root],
        }
    }
}

impl Iterator for DirWalkerIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(path) = self.stack.pop() {
            if path.is_dir() && !path.is_symlink() {
                let entries = match fs::read_dir(&path) {
                    Ok(entries) => entries,
                    Err(_) => continue,
                };

                entries
                    .filter_map(|v| v.ok())
                    .for_each(|e| self.stack.push(e.path()));
            } else if path.is_file() {
                return Some(path);
            }
        }
        None
    }
}
