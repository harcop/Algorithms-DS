/// LeetCode #1830 - Minimum Number of Operations to Make String Sorted
const N: usize = 3010;
const MOD: i64 = 1_000_000_007;

fn qmi(mut a: i64, mut k: i64) -> i64 {
    let mut res = 1i64;
    while k != 0 {
        if k & 1 == 1 {
            res = res * a % MOD;
        }
        k >>= 1;
        a = a * a % MOD;
    }
    res
}

fn factorials() -> ([i64; N], [i64; N]) {
    let mut f = [0i64; N];
    let mut g = [0i64; N];
    f[0] = 1;
    g[0] = 1;
    for i in 1..N {
        f[i] = f[i - 1] * i as i64 % MOD;
        g[i] = qmi(f[i], MOD - 2);
    }
    (f, g)
}

fn make_string_sorted(s: String) -> i32 {
    let (f, g) = factorials();
    let s: Vec<u8> = s.bytes().collect();
    let n = s.len();
    let mut cnt = [0i32; 26];
    for &c in &s {
        cnt[(c - b'a') as usize] += 1;
    }

    let mut ans = 0i64;
    for (i, &c) in s.iter().enumerate() {
        let ci = (c - b'a') as usize;
        let m: i32 = cnt[..ci].iter().sum();
        let mut t = m as i64 * f[n - i - 1] % MOD;
        for &v in &cnt {
            t = t * g[v as usize] % MOD;
        }
        ans = (ans + t) % MOD;
        cnt[ci] -= 1;
    }
    ans as i32
}

fn main() {
    println!("{}", make_string_sorted("cba".to_string()));
}

#[cfg(test)]
mod tests {
    use super::make_string_sorted;

    #[test]
    fn example_one() {
        assert_eq!(make_string_sorted("cba".to_string()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(make_string_sorted("aabaa".to_string()), 2);
    }
}
