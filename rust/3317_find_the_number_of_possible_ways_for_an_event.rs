/// LeetCode #3317 - Find the Number of Possible Ways for an Event
fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let x = x as usize;
    let y = y as i64;
    let mut f = vec![vec![0i64; x + 1]; n + 1];
    f[0][0] = 1;
    for i in 1..=n {
        for j in 1..=x {
            f[i][j] = (f[i - 1][j] * j as i64
                + f[i - 1][j - 1] * (x as i64 - (j as i64 - 1)))
                % MOD;
        }
    }
    let mut ans = 0i64;
    let mut p = 1i64;
    for j in 1..=x {
        p = p * y % MOD;
        ans = (ans + f[n][j] * p) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", number_of_ways(1, 2, 3));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example1() {
        assert_eq!(number_of_ways(1, 2, 3), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_ways(5, 2, 1), 32);
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_ways(3, 3, 4), 684);
    }
}
