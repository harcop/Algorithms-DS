/// LeetCode #1429 - First Unique Number
use std::collections::{HashMap, VecDeque};

struct FirstUnique {
    count: HashMap<i32, i32>,
    uniq: VecDeque<i32>,
}

impl FirstUnique {
    fn new(nums: Vec<i32>) -> Self {
        let mut s = Self {
            count: HashMap::new(),
            uniq: VecDeque::new(),
        };
        for x in nums {
            s.add(x);
        }
        s
    }

    fn show_first_unique(&mut self) -> i32 {
        while let Some(&front) = self.uniq.front() {
            if self.count.get(&front).copied().unwrap_or(0) == 1 {
                return front;
            }
            self.uniq.pop_front();
        }
        -1
    }

    fn add(&mut self, value: i32) {
        let c = self.count.entry(value).or_insert(0);
        *c += 1;
        if *c == 1 {
            self.uniq.push_back(value);
        }
    }
}

fn main() {
    let mut f = FirstUnique::new(vec![2, 3, 5]);
    println!("{}", f.show_first_unique());
}

#[cfg(test)]
mod tests {
    use super::FirstUnique;

    #[test]
    fn example_one() {
        let mut f = FirstUnique::new(vec![2, 3, 5]);
        assert_eq!(f.show_first_unique(), 2);
        f.add(5);
        assert_eq!(f.show_first_unique(), 2);
        f.add(2);
        assert_eq!(f.show_first_unique(), 3);
        f.add(3);
        assert_eq!(f.show_first_unique(), -1);
    }
}

