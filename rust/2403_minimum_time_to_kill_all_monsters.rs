/// LeetCode #2403 - Minimum Time to Kill All Monsters
fn minimum_time(power: Vec<i32>) -> i64 {
    let n = power.len();
    let size = 1usize << n;
    let mut dp = vec![i64::MAX / 4; size];
    dp[0] = 0;

    for mask in 0..size {
        let gain = mask.count_ones() as i64 + 1;
        for i in 0..n {
            if (mask >> i) & 1 == 0 {
                let days = (power[i] as i64 + gain - 1) / gain;
                let next = mask | (1 << i);
                dp[next] = dp[next].min(dp[mask] + days);
            }
        }
    }

    dp[size - 1]
}

fn main() {
    println!("{}", minimum_time(vec![3, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::minimum_time;

    #[test]
    fn example_one() {
        assert_eq!(minimum_time(vec![3, 1, 4]), 4);
    }

    #[test]
    fn single_monster() {
        assert_eq!(minimum_time(vec![10]), 10);
    }
}
