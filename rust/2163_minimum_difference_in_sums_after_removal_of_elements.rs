/// LeetCode #2163 - Minimum Difference in Sums After Removal of Elements
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn minimum_difference(nums: Vec<i32>) -> i64 {
    let n = nums.len() / 3;
    let m = nums.len();

    let mut left = vec![0i64; m];
    let mut max_heap = BinaryHeap::new();
    let mut sum = 0i64;
    for (i, &x) in nums.iter().enumerate() {
        max_heap.push(x);
        sum += x as i64;
        if max_heap.len() > n {
            sum -= max_heap.pop().unwrap() as i64;
        }
        if max_heap.len() == n {
            left[i] = sum;
        }
    }

    let mut right = vec![0i64; m];
    let mut min_heap = BinaryHeap::new();
    sum = 0;
    for i in (0..m).rev() {
        min_heap.push(Reverse(nums[i]));
        sum += nums[i] as i64;
        if min_heap.len() > n {
            sum -= min_heap.pop().unwrap().0 as i64;
        }
        if min_heap.len() == n {
            right[i] = sum;
        }
    }

    let mut ans = i64::MAX;
    for i in n - 1..2 * n {
        ans = ans.min(left[i] - right[i + 1]);
    }
    ans
}

fn main() {
    println!("{}", minimum_difference(vec![3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimum_difference;

    #[test]
    fn example_one() {
        assert_eq!(minimum_difference(vec![3, 1, 2]), -1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_difference(vec![7, 9, 5, 8, 1, 3]), 1);
    }
}
