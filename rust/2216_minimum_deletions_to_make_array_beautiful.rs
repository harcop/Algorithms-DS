/// LeetCode #2216 - Minimum Deletions to Make Array Beautiful
fn min_deletion(nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    for i in 0..nums.len().saturating_sub(1) {
        if nums[i] == nums[i + 1] && (i as i32 - ans) % 2 == 0 {
            ans += 1;
        }
    }
    ans + ((nums.len() as i32 - ans) & 1)
}

fn main() {
    println!("{}", min_deletion(vec![1, 1, 2, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::min_deletion;

    #[test]
    fn example_one() {
        assert_eq!(min_deletion(vec![1, 1, 2, 3, 5]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_deletion(vec![1, 1, 2, 2, 3, 3]), 2);
    }
}
