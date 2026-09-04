/// LeetCode #1172 - Dinner Plate Stacks
use std::collections::BTreeSet;

struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    available: BTreeSet<usize>,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        DinnerPlates {
            capacity: capacity as usize,
            stacks: Vec::new(),
            available: BTreeSet::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if self.available.is_empty() {
            self.stacks.push(Vec::new());
            self.available.insert(self.stacks.len() - 1);
        }
        let i = *self.available.iter().next().unwrap();
        self.stacks[i].push(val);
        if self.stacks[i].len() == self.capacity {
            self.available.remove(&i);
        }
    }

    fn pop(&mut self) -> i32 {
        while !self.stacks.is_empty() && self.stacks.last().unwrap().is_empty() {
            let i = self.stacks.len() - 1;
            self.available.remove(&i);
            self.stacks.pop();
        }
        if self.stacks.is_empty() {
            return -1;
        }
        let i = self.stacks.len() - 1;
        self.pop_at_stack(i as i32)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let i = index as usize;
        if i >= self.stacks.len() || self.stacks[i].is_empty() {
            return -1;
        }
        let v = self.stacks[i].pop().unwrap();
        self.available.insert(i);
        v
    }
}

fn main() {
    let mut d = DinnerPlates::new(2);
    d.push(1);
    d.push(2);
    println!("{}", d.pop());
}

#[cfg(test)]
mod tests {
    use super::DinnerPlates;

    #[test]
    fn example() {
        let mut d = DinnerPlates::new(2);
        d.push(1);
        d.push(2);
        d.push(3);
        d.push(4);
        d.push(5);
        assert_eq!(d.pop_at_stack(0), 2);
        d.push(20);
        d.push(21);
        assert_eq!(d.pop_at_stack(0), 20);
        assert_eq!(d.pop_at_stack(2), 21);
        assert_eq!(d.pop(), 5);
        assert_eq!(d.pop(), 4);
        assert_eq!(d.pop(), 3);
        assert_eq!(d.pop(), 1);
        assert_eq!(d.pop(), -1);
    }
}
