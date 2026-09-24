/// LeetCode #3916 - Number of ZigZag Arrays III
const MOD: i64 = 1_000_000_007;

fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    let n = n as usize;
    let m = (r - l + 1) as i64;
    let mut fact = vec![1i64; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    let mut inv_fact = vec![1i64; n + 1];
    inv_fact[n] = mod_pow(fact[n], MOD - 2);
    for i in (1..=n).rev() {
        inv_fact[i - 1] = inv_fact[i] * i as i64 % MOD;
    }
    let comb = |mm: i64, k: usize| -> i64 {
        if k == 0 {
            return 1;
        }
        let mut res = 1i64;
        for i in 0..k {
            res = res * ((mm - i as i64) % MOD) % MOD;
        }
        res * inv_fact[k] % MOD
    };

    let mut up = vec![0i64; n + 1];
    let mut down = vec![0i64; n + 1];
    up[0] = 1;
    down[0] = 1;
    for _ in 0..n - 1 {
        let mut next_up = vec![0i64; n + 1];
        for k in 0..n {
            if down[k] != 0 {
                next_up[k + 1] = (next_up[k + 1] + down[k]) % MOD;
            }
        }
        let mut next_down = vec![0i64; n + 1];
        let mut constant = 0i64;
        for k in 0..n {
            if up[k] == 0 {
                continue;
            }
            constant = (constant + up[k] * comb(m, k + 1)) % MOD;
            next_down[k] = (next_down[k] - up[k]).rem_euclid(MOD);
            next_down[k + 1] = (next_down[k + 1] - up[k]).rem_euclid(MOD);
        }
        next_down[0] = (next_down[0] + constant).rem_euclid(MOD);
        up = next_up;
        down = next_down;
    }
    let mut ans = 0i64;
    for k in 0..n {
        if up[k] == 0 && down[k] == 0 {
            continue;
        }
        let c = comb(m, k + 1);
        ans = (ans + (up[k] + down[k]) % MOD * c) % MOD;
    }
    ans as i32
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r = 1i64;
    a %= MOD;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        e >>= 1;
    }
    r
}

fn main() {
    println!("{}", zig_zag_arrays(3, 4, 5));
}

#[cfg(test)]
mod tests {
    use super::zig_zag_arrays;

    #[test]
    fn example1() {
        assert_eq!(zig_zag_arrays(3, 4, 5), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(zig_zag_arrays(3, 1, 3), 10);
    }
}
