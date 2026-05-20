/// LeetCode #1167 - Minimum Cost to Connect Sticks
use std::collections::BinaryHeap;

fn connect_sticks(sticks: Vec<i32>) -> i32 {
    let mut heap = BinaryHeap::new();
    for s in sticks {
        heap.push(-s);
    }
    let mut cost = 0i32;
    while heap.len() > 1 {
        let a = -heap.pop().unwrap();
        let b = -heap.pop().unwrap();
        let sum = a + b;
        cost += sum;
        heap.push(-sum);
    }
    cost
}

fn main() {
    println!("{}", connect_sticks(vec![2, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::connect_sticks;

    #[test]
    fn example_one() {
        assert_eq!(connect_sticks(vec![2, 4, 3]), 14);
    }

    #[test]
    fn example_two() {
        assert_eq!(connect_sticks(vec![1, 8, 3, 5]), 30);
    }
}
