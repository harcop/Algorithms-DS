/// LeetCode #3763 - Maximum Total Sum with Threshold Constraints (premium)
use std::collections::BinaryHeap;

fn max_sum(nums: Vec<i32>, threshold: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by_key(|&i| threshold[i]);
    let mut pq = BinaryHeap::new();
    let mut ans = 0i64;
    let mut i = 0;
    let mut step = 1;
    loop {
        while i < n && threshold[idx[i]] <= step {
            pq.push(nums[idx[i]]);
            i += 1;
        }
        match pq.pop() {
            Some(x) => {
                ans += x as i64;
                step += 1;
            }
            None => break,
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_sum(vec![1, 10, 4, 2, 1, 6], vec![5, 1, 5, 5, 2, 2])
    );
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example1() {
        assert_eq!(
            max_sum(vec![1, 10, 4, 2, 1, 6], vec![5, 1, 5, 5, 2, 2]),
            17
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_sum(vec![4, 1, 5, 2, 3], vec![3, 3, 2, 3, 3]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(max_sum(vec![2, 6, 10, 13], vec![2, 1, 1, 1]), 31);
    }
}
