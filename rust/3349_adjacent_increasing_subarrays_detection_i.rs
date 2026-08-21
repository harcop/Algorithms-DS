/// LeetCode #3349 - Adjacent Increasing Subarrays Detection I
fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    let mut mx = 0;
    let mut pre = 0;
    let mut cur = 0;
    for i in 0..n {
        cur += 1;
        if i == n - 1 || nums[i] >= nums[i + 1] {
            mx = mx.max(cur / 2).max(pre.min(cur));
            pre = cur;
            cur = 0;
        }
    }
    mx >= k
}

fn main() {
    println!(
        "{}",
        has_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3)
    );
}

#[cfg(test)]
mod tests {
    use super::has_increasing_subarrays;

    #[test]
    fn example1() {
        assert!(has_increasing_subarrays(
            vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1],
            3
        ));
    }

    #[test]
    fn example2() {
        assert!(!has_increasing_subarrays(
            vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7],
            5
        ));
    }
}
