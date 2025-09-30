use std::{collections::VecDeque, fs::read_dir, path::PathBuf};

pub struct FileWalker {
    file_queue: VecDeque<PathBuf>,
}

impl FileWalker {
    pub fn from(base_path: PathBuf, depth: Option<u32>) -> Self {
        let mut file_queue = VecDeque::new();
        Self::walk_dir(base_path, &mut file_queue, 0, depth.unwrap_or(10));
        Self { file_queue }
    }

    fn walk_dir(path: PathBuf, file_queue: &mut VecDeque<PathBuf>, depth: u32, max_depth: u32) {
        if let Ok(mut dir) = read_dir(path) {
            while let Some(entry) = dir.next() {
                // TODO Return Err use ?
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        // TODO No recursion
                        if depth < max_depth {
                            Self::walk_dir(path, file_queue, depth + 1, max_depth);
                        }
                    } else if path.is_file() {
                        file_queue.push_back(path);
                    }
                }
            }
        }
    }
}

impl IntoIterator for FileWalker {
    type Item = PathBuf;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.file_queue.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walking() {
        let walker = FileWalker::from(".".into(), Some(0));
        let files = walker.into_iter();
        let number_of_files = files.count();
        assert_eq!(number_of_files, 9);
    }
}
