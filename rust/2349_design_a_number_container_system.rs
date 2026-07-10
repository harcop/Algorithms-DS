/// LeetCode #2349 - Design a Number Container System
use std::collections::{BTreeSet, HashMap};

struct NumberContainers {
    d: HashMap<i32, i32>,
    g: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
    fn new() -> Self {
        NumberContainers {
            d: HashMap::new(),
            g: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        if let Some(&old) = self.d.get(&index) {
            if let Some(set) = self.g.get_mut(&old) {
                set.remove(&index);
            }
        }
        self.d.insert(index, number);
        self.g.entry(number).or_default().insert(index);
    }

    fn find(&self, number: i32) -> i32 {
        match self.g.get(&number) {
            Some(set) if !set.is_empty() => *set.iter().next().unwrap(),
            _ => -1,
        }
    }
}

fn main() {
    let nc = NumberContainers::new();
    println!("{}", nc.find(10));
}

#[cfg(test)]
mod tests {
    use super::NumberContainers;

    #[test]
    fn example_sequence() {
        let mut nc = NumberContainers::new();
        assert_eq!(nc.find(10), -1);
        nc.change(2, 10);
        nc.change(1, 10);
        nc.change(3, 10);
        nc.change(5, 10);
        assert_eq!(nc.find(10), 1);
        nc.change(1, 20);
        assert_eq!(nc.find(10), 2);
    }
}
