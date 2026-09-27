/// LeetCode #3979 - Maximum Valid Pair Sum
fn max_valid_pair_sum(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut ans = i32::MIN;
    let mut best = i32::MIN;
    for j in k..nums.len() {
        best = best.max(nums[j - k]);
        ans = ans.max(best + nums[j]);
    }
    ans
}

fn main() {
    println!("{}", max_valid_pair_sum(vec![1, 3, 5, 2, 8], 2));
}

#[cfg(test)]
mod tests {
    use super::max_valid_pair_sum;

    #[test]
    fn example1() {
        assert_eq!(max_valid_pair_sum(vec![1, 3, 5, 2, 8], 2), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(max_valid_pair_sum(vec![5, 1, 9], 1), 14);
    }
}
