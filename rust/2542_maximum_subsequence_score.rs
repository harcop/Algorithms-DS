/// LeetCode #2542 - Maximum Subsequence Score
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut pairs: Vec<(i32, i32)> = nums2.into_iter().zip(nums1.into_iter()).collect();
    pairs.sort_by(|a, b| b.0.cmp(&a.0));

    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut sum = 0i64;
    let mut ans = 0i64;

    for (num2, num1) in pairs {
        min_heap.push(Reverse(num1));
        sum += num1 as i64;
        if min_heap.len() > k {
            if let Some(Reverse(v)) = min_heap.pop() {
                sum -= v as i64;
            }
        }
        if min_heap.len() == k {
            ans = ans.max(sum * num2 as i64);
        }
    }
    ans
}

fn main() {
    println!("{}", max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score(vec![1, 3, 3, 2], vec![2, 1, 3, 4], 3), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score(vec![4, 2, 3, 1, 1], vec![7, 5, 10, 9, 6], 1), 30);
    }
}
