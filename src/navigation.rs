use std::path::{Path, PathBuf};

/// Ordered list of image file paths with cursor tracking.
pub struct FileList {
    files: Vec<PathBuf>,
    cursor: usize,
}

impl FileList {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self { files, cursor: 0 }
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub fn current_index(&self) -> usize {
        self.cursor
    }

    #[allow(dead_code)]
    pub fn current_path(&self) -> Option<&Path> {
        self.files.get(self.cursor).map(|p| p.as_path())
    }

    /// Advance to next image. Wraps around to first.
    pub fn next(&mut self) -> Option<&Path> {
        if self.files.is_empty() {
            return None;
        }
        self.cursor = (self.cursor + 1) % self.files.len();
        Some(&self.files[self.cursor])
    }

    /// Go to previous image. Wraps around to last.
    pub fn previous(&mut self) -> Option<&Path> {
        if self.files.is_empty() {
            return None;
        }
        self.cursor = if self.cursor == 0 {
            self.files.len() - 1
        } else {
            self.cursor - 1
        };
        Some(&self.files[self.cursor])
    }

    /// Jump to a specific index. Clamped to valid range.
    #[allow(dead_code)]
    pub fn go_to(&mut self, index: usize) -> Option<&Path> {
        if self.files.is_empty() {
            return None;
        }
        self.cursor = index.min(self.files.len() - 1);
        Some(&self.files[self.cursor])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let mut list = FileList::new(vec![]);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert!(list.current_path().is_none());
        assert!(list.next().is_none());
        assert!(list.previous().is_none());
    }

    #[test]
    fn test_single_file() {
        let mut list = FileList::new(vec![PathBuf::from("a.png")]);
        assert_eq!(list.current_path().unwrap(), Path::new("a.png"));
        // Next/prev should still be the same file (wrap around)
        list.next();
        assert_eq!(list.current_path().unwrap(), Path::new("a.png"));
        list.previous();
        assert_eq!(list.current_path().unwrap(), Path::new("a.png"));
    }

    #[test]
    fn test_navigation_wraps() {
        let mut list = FileList::new(vec![
            PathBuf::from("1.png"),
            PathBuf::from("2.png"),
            PathBuf::from("3.png"),
        ]);
        assert_eq!(list.current_index(), 0);

        list.next();
        assert_eq!(list.current_index(), 1);

        list.next();
        assert_eq!(list.current_index(), 2);

        list.next(); // wraps
        assert_eq!(list.current_index(), 0);

        list.previous(); // wraps back
        assert_eq!(list.current_index(), 2);
    }

    #[test]
    fn test_go_to_clamped() {
        let mut list = FileList::new(vec![
            PathBuf::from("a.png"),
            PathBuf::from("b.png"),
        ]);
        list.go_to(999);
        assert_eq!(list.current_index(), 1); // clamped to last
        list.go_to(0);
        assert_eq!(list.current_index(), 0);
    }
}
