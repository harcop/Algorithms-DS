/// LeetCode #3350 - Adjacent Increasing Subarrays Detection II
fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut pre = 0;
    let mut cur = 0;
    for i in 0..n {
        cur += 1;
        if i == n - 1 || nums[i] >= nums[i + 1] {
            ans = ans.max(cur / 2).max(pre.min(cur));
            pre = cur;
            cur = 0;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::max_increasing_subarrays;

    #[test]
    fn example1() {
        assert_eq!(
            max_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_increasing_subarrays(vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7]),
            2
        );
    }
}
