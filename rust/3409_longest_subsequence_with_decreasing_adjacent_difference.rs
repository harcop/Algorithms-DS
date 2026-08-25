/// LeetCode #3409 - Longest Subsequence With Decreasing Adjacent Difference
fn longest_subsequence(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap() as usize;
    let mut dp = vec![vec![0i32; mx + 1]; mx + 1];
    let mut ans = 2;
    for &num in &nums {
        let num = num as usize;
        let mut ndp = dp[num].clone();
        for prev in 1..=mx {
            let d = num.abs_diff(prev);
            ndp[d] = ndp[d].max(dp[prev][d] + 1);
        }
        for j in (0..mx).rev() {
            ndp[j] = ndp[j].max(ndp[j + 1]);
        }
        dp[num] = ndp;
        ans = ans.max(dp[num][0]);
    }
    ans
}

fn main() {
    println!("{}", longest_subsequence(vec![16, 6, 3]));
}

#[cfg(test)]
mod tests {
    use super::longest_subsequence;

    #[test]
    fn example1() {
        assert_eq!(longest_subsequence(vec![16, 6, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subsequence(vec![6, 5, 3, 4, 2, 1]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_subsequence(vec![10, 20, 10, 19, 10, 20]), 5);
    }
}
