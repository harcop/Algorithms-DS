/// LeetCode #703 - Kth Largest Element in a Stream
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct KthLargest { k: usize, heap: BinaryHeap<Reverse<i32>> }

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut s = Self { k: k as usize, heap: BinaryHeap::new() };
        for x in nums { s.add(x); }
        s
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        while self.heap.len() > self.k { self.heap.pop(); }
        self.heap.peek().unwrap().0
    }
}

fn main() {
    let mut k = KthLargest::new(3, vec![4,5,8,2]);
    println!("{}", k.add(3));
}

#[cfg(test)]
mod tests {
    use super::KthLargest;

    #[test]
    fn example_one() {
        let mut k = KthLargest::new(3, vec![4,5,8,2]);
        assert_eq!(k.add(3), 4);
        assert_eq!(k.add(5), 5);
        assert_eq!(k.add(10), 5);
        assert_eq!(k.add(9), 8);
        assert_eq!(k.add(4), 8);
    }
}
