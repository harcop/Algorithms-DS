/// LeetCode #2563 - Count the Number of Fair Pairs
fn count_less(nums: &[i32], sum: i64) -> i64 {
    let mut res = 0i64;
    let mut i = 0usize;
    let mut j = nums.len() - 1;
    while i < j {
        while i < j && nums[i] as i64 + nums[j] as i64 > sum {
            j -= 1;
        }
        res += (j - i) as i64;
        i += 1;
    }
    res
}

fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    nums.sort_unstable();
    count_less(&nums, upper as i64) - count_less(&nums, (lower - 1) as i64)
}

fn main() {
    println!("{}", count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6));
}

#[cfg(test)]
mod tests {
    use super::count_fair_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_fair_pairs(vec![1, 7, 9, 2, 5], 11, 11), 1);
    }
}
