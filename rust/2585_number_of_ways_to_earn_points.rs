/// LeetCode #2585 - Number of Ways to Earn Points
const MOD: i32 = 1_000_000_007;

fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
    let n = types.len();
    let target = target as usize;
    let mut f = vec![vec![0; target + 1]; n + 1];
    f[0][0] = 1;
    for i in 1..=n {
        let count = types[i - 1][0];
        let marks = types[i - 1][1];
        for j in 0..=target {
            for k in 0..=count {
                let cost = (k * marks) as usize;
                if j >= cost {
                    f[i][j] = (f[i][j] + f[i - 1][j - cost]) % MOD;
                }
            }
        }
    }
    f[n][target]
}

fn main() {
    println!(
        "{}",
        ways_to_reach_target(6, vec![vec![6, 1], vec![3, 2], vec![2, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::ways_to_reach_target;

    #[test]
    fn example_one() {
        assert_eq!(
            ways_to_reach_target(6, vec![vec![6, 1], vec![3, 2], vec![2, 3]]),
            7
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            ways_to_reach_target(5, vec![vec![50, 1], vec![50, 2], vec![50, 5]]),
            4
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            ways_to_reach_target(18, vec![vec![6, 1], vec![3, 2], vec![2, 3]]),
            1
        );
    }
}
