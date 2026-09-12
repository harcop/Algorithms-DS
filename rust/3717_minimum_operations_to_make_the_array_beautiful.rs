/// LeetCode #3717 - Minimum Operations to Make the Array Beautiful
fn min_operations(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap();
    if mx == 1 {
        return 0;
    }
    const INF: i32 = i32::MAX / 4;
    let lim = (2 * mx - 2) as usize;
    let mut dp = vec![INF; lim + 1];
    dp[nums[0] as usize] = 0;
    for i in 1..nums.len() {
        let mut new_dp = vec![INF; lim + 1];
        let ni = nums[i] as i32;
        for x in 1..=lim {
            if dp[x] == INF {
                continue;
            }
            let x = x as i32;
            let start = (ni + x - 1) / x;
            let end = lim as i32 / x;
            for j in start..=end {
                let val = (j * x) as usize;
                new_dp[val] = new_dp[val].min(dp[x as usize] + (j * x - ni));
            }
        }
        dp = new_dp;
    }
    *dp.iter().min().unwrap()
}

fn main() {
    println!("{}", min_operations(vec![3, 7, 9]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![3, 7, 9]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 1, 1]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![4]), 0);
    }
}
