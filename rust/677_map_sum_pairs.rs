/// LeetCode #677 - Map Sum Pairs
use std::collections::HashMap;

struct MapSum {
    map: HashMap<String, i32>,
}

impl MapSum {
    fn new() -> Self {
        Self { map: HashMap::new() }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.map.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.map
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix))
            .map(|(_, &v)| v)
            .sum()
    }
}

fn main() {
    let mut m = MapSum::new();
    m.insert("apple".into(), 3);
    println!("{}", m.sum("ap".into()));
}

#[cfg(test)]
mod tests {
    use super::MapSum;

    #[test]
    fn example() {
        let mut m = MapSum::new();
        m.insert("apple".into(), 3);
        assert_eq!(m.sum("ap".into()), 3);
        m.insert("app".into(), 2);
        assert_eq!(m.sum("ap".into()), 5);
        m.insert("apple".into(), 4);
        assert_eq!(m.sum("ap".into()), 6);
    }
}
