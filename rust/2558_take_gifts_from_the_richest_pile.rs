/// LeetCode #2558 - Take Gifts From the Richest Pile
use std::collections::BinaryHeap;

fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<i32> = gifts.into_iter().collect();
    for _ in 0..k {
        if let Some(max) = heap.pop() {
            heap.push((max as f64).sqrt().floor() as i32);
        }
    }
    heap.into_iter().map(|x| x as i64).sum()
}

fn main() {
    println!("{}", pick_gifts(vec![25, 64, 9, 4, 100], 4));
}

#[cfg(test)]
mod tests {
    use super::pick_gifts;

    #[test]
    fn example_one() {
        assert_eq!(pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
    }

    #[test]
    fn example_two() {
        assert_eq!(pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }
}
