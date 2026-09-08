/// LeetCode #3655 - XOR After Range Multiplication Queries II
fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    const MOD: i64 = 1_000_000_007;
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

fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = nums.len();
    let b = ((n as f64).sqrt() as usize) + 1;
    let mut events: Vec<Vec<Vec<(usize, i32)>>> = (0..=b)
        .map(|k| if k == 0 { vec![] } else { vec![Vec::new(); k] })
        .collect();

    for q in queries {
        let (l, r, k, v) = (q[0] as usize, q[1] as usize, q[2] as usize, q[3]);
        if k > b {
            let mut idx = l;
            while idx <= r {
                nums[idx] = ((nums[idx] as i64 * v as i64) % MOD) as i32;
                idx += k;
            }
        } else {
            let res = l % k;
            let t1 = (l - res) / k;
            let t2 = (r - res) / k;
            events[k][res].push((t1, v));
            if t2 + 1 <= (n - 1 - res) / k {
                let invv = mod_pow(v as i64, MOD - 2) as i32;
                events[k][res].push((t2 + 1, invv));
            }
        }
    }

    for k in 1..=b {
        for res in 0..k {
            let ev = &mut events[k][res];
            if ev.is_empty() {
                continue;
            }
            ev.sort_unstable_by_key(|p| p.0);
            let mut comp: Vec<(usize, i64)> = Vec::new();
            for &(t, val) in ev.iter() {
                if let Some(last) = comp.last_mut() {
                    if last.0 == t {
                        last.1 = last.1 * val as i64 % MOD;
                        continue;
                    }
                }
                comp.push((t, val as i64));
            }
            let mut cur = 1i64;
            let mut ptr = 0usize;
            let mut t = 0usize;
            let mut idx = res;
            while idx < n {
                while ptr < comp.len() && comp[ptr].0 == t {
                    cur = cur * comp[ptr].1 % MOD;
                    ptr += 1;
                }
                nums[idx] = ((nums[idx] as i64 * cur) % MOD) as i32;
                idx += k;
                t += 1;
            }
        }
    }

    nums.into_iter().fold(0, |a, x| a ^ x)
}

fn main() {
    println!("{}", xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]]));
}

#[cfg(test)]
mod tests {
    use super::xor_after_queries;

    #[test]
    fn example1() {
        assert_eq!(xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(
            xor_after_queries(vec![2, 3, 1, 5, 4], vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]]),
            31
        );
    }
}
