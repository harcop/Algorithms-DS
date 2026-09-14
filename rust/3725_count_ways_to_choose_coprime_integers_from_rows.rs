/// LeetCode #3725 - Count Ways to Choose Coprime Integers from Rows
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn count_coprime(mat: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut dp = [0i64; 151];
    for &x in &mat[0] {
        dp[x as usize] += 1;
    }
    for row in mat.iter().skip(1) {
        let mut ndp = [0i64; 151];
        for g in 1..=150 {
            if dp[g] == 0 {
                continue;
            }
            for &x in row {
                let ng = gcd(g as i32, x) as usize;
                ndp[ng] = (ndp[ng] + dp[g]) % MOD;
            }
        }
        dp = ndp;
    }
    dp[1] as i32
}

fn main() {
    println!("{}", count_coprime(vec![vec![1, 2], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::count_coprime;

    #[test]
    fn example1() {
        assert_eq!(count_coprime(vec![vec![1, 2], vec![3, 4]]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_coprime(vec![vec![2, 2], vec![2, 2]]), 0);
    }
}
