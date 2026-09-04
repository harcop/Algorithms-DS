/// LeetCode #1166 - Design File System
use std::collections::HashMap;

struct FileSystem {
    paths: HashMap<String, i32>,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            paths: HashMap::new(),
        }
    }

    fn create_path(&mut self, path: String, value: i32) -> bool {
        if path.is_empty() || path == "/" || self.paths.contains_key(&path) {
            return false;
        }
        if let Some(idx) = path.rfind('/') {
            if idx > 0 {
                let parent = &path[..idx];
                if !self.paths.contains_key(parent) {
                    return false;
                }
            }
        }
        self.paths.insert(path, value);
        true
    }

    fn get(&self, path: String) -> i32 {
        *self.paths.get(&path).unwrap_or(&-1)
    }
}

fn main() {
    let mut fs = FileSystem::new();
    println!("{}", fs.create_path("/a".into(), 1));
    println!("{}", fs.get("/a".into()));
}

#[cfg(test)]
mod tests {
    use super::FileSystem;

    #[test]
    fn example_one() {
        let mut fs = FileSystem::new();
        assert!(fs.create_path("/a".into(), 1));
        assert_eq!(fs.get("/a".into()), 1);
    }

    #[test]
    fn example_two() {
        let mut fs = FileSystem::new();
        assert!(fs.create_path("/leet".into(), 1));
        assert!(fs.create_path("/leet/code".into(), 2));
        assert_eq!(fs.get("/leet/code".into()), 2);
        assert!(!fs.create_path("/c/d".into(), 1));
        assert_eq!(fs.get("/c".into()), -1);
    }
}
