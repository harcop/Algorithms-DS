/// LeetCode #3627 - Maximum Median Sum of Subsequences of Size 3
fn maximum_median_sum(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let n = nums.len();
    let mut ans = 0i64;
    let mut i = n / 3;
    while i < n {
        ans += nums[i] as i64;
        i += 2;
    }
    ans
}

fn main() {
    println!("{}", maximum_median_sum(vec![2, 1, 3, 2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::maximum_median_sum;

    #[test]
    fn example1() {
        assert_eq!(maximum_median_sum(vec![2, 1, 3, 2, 1, 3]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_median_sum(vec![1, 1, 10, 10, 10, 10]), 20);
    }
}
