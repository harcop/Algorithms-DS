/// LeetCode #3376 - Minimum Time to Break Locks I
fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
    let n = strength.len();
    let m = 1 << n;
    let mut dp = vec![i32::MAX / 2; m];
    dp[0] = 0;
    for mask in 0..m {
        let x = 1 + mask.count_ones() as i32 * k;
        for j in 0..n {
            if mask & (1 << j) == 0 {
                let nmask = mask | (1 << j);
                let add = (strength[j] + x - 1) / x;
                dp[nmask] = dp[nmask].min(dp[mask] + add);
            }
        }
    }
    dp[m - 1]
}

fn main() {
    println!("{}", find_minimum_time(vec![3, 4, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::find_minimum_time;

    #[test]
    fn example1() {
        assert_eq!(find_minimum_time(vec![3, 4, 1], 1), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(find_minimum_time(vec![2, 5, 4], 2), 5);
    }
}
