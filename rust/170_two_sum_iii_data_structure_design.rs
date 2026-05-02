/// LeetCode #170 - Two Sum III - Data structure design
use std::collections::HashMap;

pub struct TwoSum {
    cnt: HashMap<i32, i32>,
}

impl TwoSum {
    fn new() -> Self {
        TwoSum {
            cnt: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        *self.cnt.entry(number).or_insert(0) += 1;
    }

    fn find(&self, value: i32) -> bool {
        for (&k, &c) in &self.cnt {
            let need = value - k;
            if need == k {
                if c > 1 {
                    return true;
                }
            } else if self.cnt.contains_key(&need) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let mut t = TwoSum::new();
    t.add(1);
    t.add(3);
    t.add(5);
    println!("{}", t.find(4));
}

#[cfg(test)]
mod tests {
    use super::TwoSum;

    #[test]
    fn example() {
        let mut t = TwoSum::new();
        t.add(1);
        t.add(3);
        t.add(5);
        assert!(t.find(4));
        assert!(!t.find(7));
    }
}
