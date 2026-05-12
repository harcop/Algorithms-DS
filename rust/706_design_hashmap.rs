/// LeetCode #706 - Design HashMap
struct MyHashMap { buckets: Vec<Vec<(i32, i32)>> }
impl MyHashMap {
    fn new() -> Self { Self { buckets: vec![vec![]; 769] } }
    fn h(&self, k: i32) -> usize { (k as usize) % self.buckets.len() }
    fn put(&mut self, key: i32, value: i32) {
        let i = self.h(key);
        for kv in self.buckets[i].iter_mut() {
            if kv.0 == key { kv.1 = value; return; }
        }
        self.buckets[i].push((key, value));
    }
    fn get(&self, key: i32) -> i32 {
        let i = self.h(key);
        for kv in &self.buckets[i] { if kv.0 == key { return kv.1; } }
        -1
    }
    fn remove(&mut self, key: i32) {
        let i = self.h(key);
        self.buckets[i].retain(|kv| kv.0 != key);
    }
}

fn main() {
    let mut m = MyHashMap::new();
    m.put(1, 100);
    println!("{}", m.get(1));
}

#[cfg(test)]
mod tests {
    use super::MyHashMap;

    #[test]
    fn example() {
        let mut m = MyHashMap::new();
        m.put(1, 1);
        m.put(2, 2);
        assert_eq!(m.get(1), 1);
        assert_eq!(m.get(3), -1);
        m.put(2, 1);
        assert_eq!(m.get(2), 1);
        m.remove(2);
        assert_eq!(m.get(2), -1);
    }
}
