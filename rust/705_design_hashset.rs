/// LeetCode #705 - Design HashSet
struct MyHashSet { buckets: Vec<Vec<i32>> }
impl MyHashSet {
    fn new() -> Self { Self { buckets: vec![vec![]; 769] } }
    fn h(&self, k: i32) -> usize { (k as usize) % self.buckets.len() }
    fn add(&mut self, key: i32) {
        let i = self.h(key);
        if !self.buckets[i].contains(&key) { self.buckets[i].push(key); }
    }
    fn remove(&mut self, key: i32) {
        let i = self.h(key);
        self.buckets[i].retain(|&x| x != key);
    }
    fn contains(&self, key: i32) -> bool {
        let i = self.h(key);
        self.buckets[i].contains(&key)
    }
}

fn main() {
    let mut s = MyHashSet::new();
    s.add(1);
    println!("{}", s.contains(1));
}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn example() {
        let mut s = MyHashSet::new();
        s.add(1);
        s.add(2);
        assert!(s.contains(1));
        assert!(!s.contains(3));
        s.add(2);
        assert!(s.contains(2));
        s.remove(2);
        assert!(!s.contains(2));
    }
}
