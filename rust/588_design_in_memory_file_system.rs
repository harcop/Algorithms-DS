/// LeetCode #588 - Design In-Memory File System
use std::collections::BTreeMap;

enum Entry {
    Dir(BTreeMap<String, Entry>),
    File(String),
}

struct FileSystem {
    root: Entry,
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            root: Entry::Dir(BTreeMap::new()),
        }
    }

    fn parts(path: &str) -> Vec<&str> {
        path.split('/').filter(|s| !s.is_empty()).collect()
    }

    fn ls(&self, path: String) -> Vec<String> {
        let parts = Self::parts(&path);
        let mut cur = &self.root;
        for p in &parts {
            match cur {
                Entry::Dir(m) => cur = m.get(*p).unwrap(),
                Entry::File(_) => break,
            }
        }
        match cur {
            Entry::File(_) => vec![parts.last().unwrap().to_string()],
            Entry::Dir(m) => m.keys().cloned().collect(),
        }
    }

    fn mkdir(&mut self, path: String) {
        let parts = Self::parts(&path);
        let mut cur = &mut self.root;
        for p in parts {
            if let Entry::Dir(m) = cur {
                cur = m.entry(p.to_string()).or_insert_with(|| Entry::Dir(BTreeMap::new()));
            }
        }
    }

    fn add_content_to_file(&mut self, file_path: String, content: String) {
        let parts = Self::parts(&file_path);
        let n = parts.len();
        let mut cur = &mut self.root;
        for (i, p) in parts.iter().enumerate() {
            if let Entry::Dir(m) = cur {
                if i == n - 1 {
                    match m.get_mut(*p) {
                        Some(Entry::File(s)) => s.push_str(&content),
                        _ => {
                            m.insert(p.to_string(), Entry::File(content.clone()));
                        }
                    }
                    return;
                } else {
                    cur = m
                        .entry(p.to_string())
                        .or_insert_with(|| Entry::Dir(BTreeMap::new()));
                }
            }
        }
    }

    fn read_content_from_file(&self, file_path: String) -> String {
        let parts = Self::parts(&file_path);
        let mut cur = &self.root;
        for p in &parts {
            match cur {
                Entry::Dir(m) => cur = m.get(*p).unwrap(),
                Entry::File(_) => break,
            }
        }
        match cur {
            Entry::File(s) => s.clone(),
            Entry::Dir(_) => String::new(),
        }
    }
}

fn main() {
    let mut fs = FileSystem::new();
    println!("{:?}", fs.ls("/".into()));
}

#[cfg(test)]
mod tests {
    use super::FileSystem;

    #[test]
    fn example() {
        let mut fs = FileSystem::new();
        assert_eq!(fs.ls("/".into()), Vec::<String>::new());
        fs.mkdir("/a/b/c".into());
        fs.add_content_to_file("/a/b/c/d".into(), "hello".into());
        assert_eq!(fs.ls("/".into()), vec!["a".to_string()]);
        assert_eq!(fs.read_content_from_file("/a/b/c/d".into()), "hello");
    }
}
