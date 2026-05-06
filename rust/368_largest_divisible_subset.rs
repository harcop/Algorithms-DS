/// LeetCode #368 - Largest Divisible Subset
fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    nums.sort_unstable();
    let n = nums.len();
    let mut dp = vec![1usize; n];
    let mut prev = vec![usize::MAX; n];
    let mut best_i = 0usize;
    for i in 0..n {
        for j in 0..i {
            if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = j;
            }
        }
        if dp[i] > dp[best_i] {
            best_i = i;
        }
    }
    let mut out = vec![];
    let mut cur = best_i;
    loop {
        out.push(nums[cur]);
        if prev[cur] == usize::MAX {
            break;
        }
        cur = prev[cur];
    }
    out.reverse();
    out
}

fn main() {
    println!("{:?}", largest_divisible_subset(vec![1, 2, 4, 8]));
}

#[cfg(test)]
mod tests {
    use super::largest_divisible_subset;

    #[test]
    fn example_one() {
        assert_eq!(largest_divisible_subset(vec![1, 2, 3]), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_divisible_subset(vec![1, 2, 4, 8]), vec![1, 2, 4, 8]);
    }
}
