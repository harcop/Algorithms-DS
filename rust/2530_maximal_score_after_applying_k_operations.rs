/// LeetCode #2530 - Maximal Score After Applying K Operations
use std::collections::BinaryHeap;

fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
    let mut ans = 0i64;
    for _ in 0..k {
        let num = heap.pop().unwrap();
        ans += num as i64;
        heap.push((num + 2) / 3);
    }
    ans
}

fn main() {
    println!("{}", max_kelements(vec![10, 10, 10, 10, 10], 5));
}

#[cfg(test)]
mod tests {
    use super::max_kelements;

    #[test]
    fn example_one() {
        assert_eq!(max_kelements(vec![10, 10, 10, 10, 10], 5), 50);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_kelements(vec![1, 10, 3, 3, 3], 3), 17);
    }
}
