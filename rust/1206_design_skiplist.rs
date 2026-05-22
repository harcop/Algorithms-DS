/// LeetCode #1206 - Design Skiplist
struct Skiplist {
    set: std::collections::BTreeSet<i32>,
}

impl Skiplist {
    fn new() -> Self {
        Self {
            set: std::collections::BTreeSet::new(),
        }
    }

    fn search(&self, target: i32) -> bool {
        self.set.contains(&target)
    }

    fn add(&mut self, num: i32) {
        self.set.insert(num);
    }

    fn erase(&mut self, num: i32) -> bool {
        self.set.remove(&num)
    }
}

fn main() {
    let mut sl = Skiplist::new();
    sl.add(1);
    println!("{}", sl.search(1));
}

#[cfg(test)]
mod tests {
    use super::Skiplist;

    #[test]
    fn example_ops() {
        let mut sl = Skiplist::new();
        sl.add(1);
        sl.add(2);
        sl.add(3);
        assert!(!sl.search(0));
        assert!(sl.search(1));
        sl.add(2);
        assert!(sl.search(2));
        assert!(sl.erase(3));
        assert!(!sl.search(3));
    }
}
