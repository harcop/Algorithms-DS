/// LeetCode #3804 - Number of Centered Subarrays
use std::collections::HashSet;

fn centered_subarrays(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        let mut st = HashSet::new();
        let mut s = 0;
        for j in i..n {
            s += nums[j];
            st.insert(nums[j]);
            if st.contains(&s) {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", centered_subarrays(vec![-1, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::centered_subarrays;

    #[test]
    fn example1() {
        assert_eq!(centered_subarrays(vec![-1, 1, 0]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(centered_subarrays(vec![2, -3]), 2);
    }
}
