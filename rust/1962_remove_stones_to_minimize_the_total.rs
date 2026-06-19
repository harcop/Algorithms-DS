/// LeetCode #1962 - Remove Stones to Minimize the Total
use std::collections::BinaryHeap;

fn min_stone_sum(piles: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<i32> = piles.into_iter().collect();
    for _ in 0..k {
        if let Some(top) = heap.pop() {
            heap.push(top - top / 2);
        }
    }
    heap.iter().sum()
}

fn main() {
    println!("{}", min_stone_sum(vec![5, 4, 9], 2));
}

#[cfg(test)]
mod tests {
    use super::min_stone_sum;

    #[test]
    fn example_one() {
        assert_eq!(min_stone_sum(vec![5, 4, 9], 2), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_stone_sum(vec![4, 3, 6, 7], 3), 12);
    }
}
