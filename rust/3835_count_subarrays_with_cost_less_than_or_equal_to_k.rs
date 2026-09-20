/// LeetCode #3835 - Count Subarrays With Cost Less Than or Equal to K
use std::collections::VecDeque;

fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let mut ans = 0i64;
    let mut q1: VecDeque<usize> = VecDeque::new();
    let mut q2: VecDeque<usize> = VecDeque::new();
    let mut l = 0usize;
    for r in 0..nums.len() {
        let x = nums[r];
        while q1.back().map(|&i| nums[i] <= x).unwrap_or(false) {
            q1.pop_back();
        }
        while q2.back().map(|&i| nums[i] >= x).unwrap_or(false) {
            q2.pop_back();
        }
        q1.push_back(r);
        q2.push_back(r);
        while l < r
            && (nums[*q1.front().unwrap()] as i64 - nums[*q2.front().unwrap()] as i64)
                * (r as i64 - l as i64 + 1)
                > k
        {
            l += 1;
            if *q1.front().unwrap() < l {
                q1.pop_front();
            }
            if *q2.front().unwrap() < l {
                q2.pop_front();
            }
        }
        ans += r as i64 - l as i64 + 1;
    }
    ans
}

fn main() {
    println!("{}", count_subarrays(vec![1, 3, 2], 4));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_subarrays(vec![1, 3, 2], 4), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(count_subarrays(vec![5, 5, 5, 5], 0), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(count_subarrays(vec![1, 2, 3], 0), 3);
    }
}
