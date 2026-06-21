/// LeetCode #2023 - Number of Pairs of Strings With Concatenation Equal to Target
fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if i != j && format!("{}{}", nums[i], nums[j]) == target {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        num_of_pairs(vec!["777".into(), "7".into(), "77".into(), "77".into()], "7777".into())
    );
}

#[cfg(test)]
mod tests {
    use super::num_of_pairs;

    #[test]
    fn example_one() {
        assert_eq!(
            num_of_pairs(vec!["777".into(), "7".into(), "77".into(), "77".into()], "7777".into()),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            num_of_pairs(vec!["123".into(), "4".into(), "12".into(), "34".into()], "1234".into()),
            2
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(num_of_pairs(vec!["1".into(), "1".into(), "1".into()], "11".into()), 6);
    }
}
