/// LeetCode #1223 - Dice Roll Simulation
const MOD: i64 = 1_000_000_007;

fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0i64; 16]; 6];
    for i in 0..6 {
        dp[i][1] = 1;
    }
    for _ in 1..n {
        let mut ndp = vec![vec![0i64; 16]; 6];
        for i in 0..6 {
            for len in 1..=15 {
                if dp[i][len] == 0 {
                    continue;
                }
                for j in 0..6 {
                    if i == j {
                        if len < roll_max[i] as usize {
                            ndp[j][len + 1] = (ndp[j][len + 1] + dp[i][len]) % MOD;
                        }
                    } else {
                        ndp[j][1] = (ndp[j][1] + dp[i][len]) % MOD;
                    }
                }
            }
        }
        dp = ndp;
    }
    let mut ans = 0i64;
    for i in 0..6 {
        for len in 1..=15 {
            ans = (ans + dp[i][len]) % MOD;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", die_simulator(2, vec![1, 1, 1, 1, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::die_simulator;

    #[test]
    fn example_one() {
        assert_eq!(die_simulator(2, vec![1, 1, 1, 1, 1, 1]), 30);
    }

    #[test]
    fn example_two() {
        assert_eq!(die_simulator(2, vec![1, 1, 2, 2, 2, 3]), 34);
    }

    #[test]
    fn example_three() {
        assert_eq!(die_simulator(3, vec![1, 1, 1, 2, 2, 3]), 181);
    }
}
