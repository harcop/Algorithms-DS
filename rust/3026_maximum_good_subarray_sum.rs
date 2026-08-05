/// LeetCode #3026 - Maximum Good Subarray Sum
use std::collections::HashMap;

fn maximum_good_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut ans = i64::MIN;
    let mut p: HashMap<i32, i64> = HashMap::new();
    p.insert(nums[0], 0);
    let mut s = 0i64;

    for (i, &x) in nums.iter().enumerate() {
        s += x as i64;
        if let Some(&prev) = p.get(&(x - k)) {
            ans = ans.max(s - prev);
        }
        if let Some(&prev) = p.get(&(x + k)) {
            ans = ans.max(s - prev);
        }
        if i + 1 < n {
            let next = nums[i + 1];
            if !p.contains_key(&next) || p[&next] > s {
                p.insert(next, s);
            }
        }
    }

    if ans == i64::MIN {
        0
    } else {
        ans
    }
}

fn main() {
    println!("{}", maximum_good_subarray_sum(vec![1, 2, 3, 4, 5, 6], 1));
}

#[cfg(test)]
mod tests {
    use super::maximum_good_subarray_sum;

    #[test]
    fn example1() {
        assert_eq!(maximum_good_subarray_sum(vec![1, 2, 3, 4, 5, 6], 1), 11);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_good_subarray_sum(vec![-1, 3, 2, 4, 5], 3), 11);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_good_subarray_sum(vec![-1, -2, -3, -4], 2), -6);
    }
}
