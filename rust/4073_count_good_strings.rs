/// LeetCode #4073 - Count Good Strings
fn count_good_strings(n: i64) -> i32 {
    const MOD: i64 = 1_000_000_007;
    fn mul(a: [[i64; 2]; 2], b: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
        let mut c = [[0i64; 2]; 2];
        for i in 0..2 {
            for k in 0..2 {
                for j in 0..2 {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
                }
            }
        }
        c
    }

    // Good strings are exactly the strings of a and b whose every run has odd length.
    // Their count is 2 * F_n.
    let mut base = [[1i64, 1], [1, 0]];
    let mut acc = [[1i64, 0], [0, 1]];
    let mut exp = n - 1;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mul(acc, base);
        }
        base = mul(base, base);
        exp >>= 1;
    }
    let fib_n = acc[0][0];
    ((2 * fib_n) % MOD) as i32
}

fn main() {
    println!("{}", count_good_strings(4));
}

#[cfg(test)]
mod tests {
    use super::count_good_strings;

    #[test]
    fn example1() {
        assert_eq!(count_good_strings(4), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(count_good_strings(3), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(count_good_strings(2), 2);
    }

    #[test]
    fn length_one_and_large() {
        assert_eq!(count_good_strings(1), 2);
        assert_eq!(count_good_strings(1_000_000_000_000_000), 296_650_267);
    }
}
