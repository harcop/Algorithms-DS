/// LeetCode #3874 - Valid Subarrays With Exactly One Peak (premium)
fn valid_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut peaks = Vec::new();
    for i in 1..n - 1 {
        if nums[i] > nums[i - 1] && nums[i] > nums[i + 1] {
            peaks.push(i);
        }
    }
    let mut ans = 0i64;
    for j in 0..peaks.len() {
        let p = peaks[j] as i32;
        let mut left_min = (p - k).max(0);
        if j > 0 {
            left_min = left_min.max(peaks[j - 1] as i32 + 1);
        }
        let mut right_max = (p + k).min(n as i32 - 1);
        if j + 1 < peaks.len() {
            right_max = right_max.min(peaks[j + 1] as i32 - 1);
        }
        ans += (p - left_min + 1) as i64 * (right_max - p + 1) as i64;
    }
    ans
}

fn main() {
    println!("{}", valid_subarrays(vec![1, 3, 2], 1));
}

#[cfg(test)]
mod tests {
    use super::valid_subarrays;

    #[test]
    fn example1() {
        assert_eq!(valid_subarrays(vec![1, 3, 2], 1), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(valid_subarrays(vec![7, 8, 9], 2), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(valid_subarrays(vec![4, 3, 5, 1], 2), 6);
    }
}
