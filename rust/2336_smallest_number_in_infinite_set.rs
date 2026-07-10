/// LeetCode #2336 - Smallest Number in Infinite Set
use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    s: BTreeSet<i32>,
}

impl SmallestInfiniteSet {
    fn new() -> Self {
        let mut s = BTreeSet::new();
        for i in 1..=1000 {
            s.insert(i);
        }
        SmallestInfiniteSet { s }
    }

    fn pop_smallest(&mut self) -> i32 {
        let x = *self.s.iter().next().unwrap();
        self.s.remove(&x);
        x
    }

    fn add_back(&mut self, num: i32) {
        self.s.insert(num);
    }
}

fn main() {
    let mut set = SmallestInfiniteSet::new();
    set.add_back(2);
    println!("{}", set.pop_smallest());
}

#[cfg(test)]
mod tests {
    use super::SmallestInfiniteSet;

    #[test]
    fn example_sequence() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(2);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 2);
        assert_eq!(set.pop_smallest(), 3);
        set.add_back(1);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 4);
        assert_eq!(set.pop_smallest(), 5);
    }
}
