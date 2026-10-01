/// LeetCode #4039 - Sum of Decoded Numbers
fn modpow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut acc = 1i64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = acc * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    acc
}

fn sum_decoded(nums: Vec<i64>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 0i64;
    for v in nums {
        let width = (v % 10) as usize;
        let d = (v / 10).to_string();
        let x: i64 = d[..width].parse().unwrap();
        let y: i64 = d[width..].parse().unwrap();
        ans = (ans + modpow(x, y, MOD)) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_decoded(vec![231]));
}

#[cfg(test)]
mod tests {
    use super::sum_decoded;

    #[test]
    fn example1() {
        assert_eq!(sum_decoded(vec![231]), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_decoded(vec![2522, 2101]), 1649);
    }

    #[test]
    fn example3() {
        assert_eq!(sum_decoded(vec![2301]), 73741817);
    }
}
