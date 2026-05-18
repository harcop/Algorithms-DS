/// LeetCode #1046 - Last Stone Weight
use std::collections::BinaryHeap;

fn last_stone_weight(stones: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<i32> = stones.into_iter().collect();
    while heap.len() > 1 {
        let a = heap.pop().unwrap();
        let b = heap.pop().unwrap();
        if a != b {
            heap.push(a - b);
        }
    }
    heap.pop().unwrap_or(0)
}

fn main() {
    println!("{}", last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
}

#[cfg(test)]
mod tests {
    use super::last_stone_weight;

    #[test]
    fn example_one() {
        assert_eq!(last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(last_stone_weight(vec![1]), 1);
    }
}
