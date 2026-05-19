/// LeetCode #1099 - Two Sum Less Than K
fn two_sum_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut ans = -1;
    let mut i = 0usize;
    let mut j = nums.len() - 1;
    while i < j {
        let s = nums[i] + nums[j];
        if s < k {
            ans = ans.max(s);
            i += 1;
        } else {
            j -= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", two_sum_less_than_k(vec![34, 23, 1, 24, 75, 33, 54, 8], 60));
}

#[cfg(test)]
mod tests {
    use super::two_sum_less_than_k;

    #[test]
    fn example_one() {
        assert_eq!(two_sum_less_than_k(vec![34, 23, 1, 24, 75, 33, 54, 8], 60), 58);
    }

    #[test]
    fn example_two() {
        assert_eq!(two_sum_less_than_k(vec![10, 20, 30], 31), 30);
    }
}
