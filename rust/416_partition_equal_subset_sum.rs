/// LeetCode #416 - Partition Equal Subset Sum
fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 != 0 {
        return false;
    }
    let target = (sum / 2) as usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for x in nums {
        let x = x as usize;
        for s in (x..=target).rev() {
            if dp[s - x] {
                dp[s] = true;
            }
        }
    }
    dp[target]
}

fn main() {
    println!("{}", can_partition(vec![1, 5, 11, 5]));
}

#[cfg(test)]
mod tests {
    use super::can_partition;

    #[test]
    fn example_one() {
        assert!(can_partition(vec![1, 5, 11, 5]));
    }

    #[test]
    fn example_two() {
        assert!(!can_partition(vec![1, 2, 3, 5]));
    }
}
