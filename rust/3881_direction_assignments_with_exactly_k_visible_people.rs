/// LeetCode #3881 - Direction Assignments with Exactly K Visible People
fn qmi(mut a: i64, mut k: i64, p: i64) -> i64 {
    let mut res = 1i64;
    while k != 0 {
        if k & 1 == 1 {
            res = res * a % p;
        }
        k >>= 1;
        a = a * a % p;
    }
    res
}

fn count_visible_people(n: i32, pos: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut f = vec![1i64; n + 1];
    let mut g = vec![1i64; n + 1];
    for i in 1..=n {
        f[i] = f[i - 1] * i as i64 % MOD;
        g[i] = qmi(f[i], MOD - 2, MOD);
    }
    let comb = |nn: i32, kk: i32| -> i64 {
        if kk < 0 || kk > nn {
            0
        } else {
            f[nn as usize] * g[kk as usize] % MOD * g[(nn - kk) as usize] % MOD
        }
    };
    let l = pos;
    let r = n as i32 - pos - 1;
    let mut ans = 0i64;
    for a in 0..=k.min(l) {
        let b = k - a;
        if b <= r {
            ans = (ans + 2 * comb(l, a) % MOD * comb(r, b) % MOD) % MOD;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", count_visible_people(3, 1, 0));
}

#[cfg(test)]
mod tests {
    use super::count_visible_people;

    #[test]
    fn example1() {
        assert_eq!(count_visible_people(3, 1, 0), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_visible_people(3, 2, 1), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(count_visible_people(1, 0, 0), 2);
    }
}
