/// LeetCode #935 - Knight Dialer
const MOD: i64 = 1_000_000_007;

fn knight_dialer(n: i32) -> i32 {
    let moves: [Vec<usize>; 10] = [
        vec![4, 6],
        vec![6, 8],
        vec![7, 9],
        vec![4, 8],
        vec![0, 3, 9],
        vec![],
        vec![1, 7, 0],
        vec![2, 6],
        vec![1, 3],
        vec![2, 4],
    ];
    let mut dp = vec![1i64; 10];
    for _ in 1..n {
        let mut ndp = vec![0i64; 10];
        for from in 0..10 {
            for &to in &moves[from] {
                ndp[to] = (ndp[to] + dp[from]) % MOD;
            }
        }
        dp = ndp;
    }
    (dp.iter().sum::<i64>() % MOD) as i32
}

fn main() {
    println!("{}", knight_dialer(1));
}

#[cfg(test)]
mod tests {
    use super::knight_dialer;

    #[test]
    fn example_one() {
        assert_eq!(knight_dialer(1), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(knight_dialer(2), 20);
    }

    #[test]
    fn example_three() {
        assert_eq!(knight_dialer(3131), 136006598);
    }
}
