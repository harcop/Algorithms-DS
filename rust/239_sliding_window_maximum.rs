/// LeetCode #239 - Sliding Window Maximum
use std::collections::VecDeque;

fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut dq = VecDeque::new();
    let mut out = vec![];
    for (i, &x) in nums.iter().enumerate() {
        while dq.back().is_some_and(|&j| nums[j] <= x) {
            dq.pop_back();
        }
        dq.push_back(i);
        if i >= k && *dq.front().unwrap() == i - k {
            dq.pop_front();
        }
        if i + 1 >= k {
            out.push(nums[*dq.front().unwrap()]);
        }
    }
    out
}

fn main() {
    println!("{:?}", max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));
}

#[cfg(test)]
mod tests {
    use super::max_sliding_window;

    #[test]
    fn example_one() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(max_sliding_window(vec![1], 1), vec![1]);
    }
}
