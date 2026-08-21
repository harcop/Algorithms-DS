/// LeetCode #3352 - Count K-Reducible Numbers Less Than N
fn count_k_reducible_numbers(s: String, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let s = s.into_bytes();
    let n = s.len();
    let mut ops = vec![0usize; n + 1];
    for i in 2..=n {
        ops[i] = ops[i.count_ones() as usize] + 1;
    }
    let k = k as usize;
    let mut memo = vec![vec![vec![-1i64; n + 1]; 2]; n];
    fn dfs(
        pos: usize,
        tight: usize,
        bits: usize,
        s: &[u8],
        ops: &[usize],
        k: usize,
        memo: &mut [Vec<Vec<i64>>],
    ) -> i64 {
        if pos == s.len() {
            if tight == 1 || bits == 0 {
                return 0;
            }
            return i64::from(ops[bits] + 1 <= k);
        }
        if memo[pos][tight][bits] != -1 {
            return memo[pos][tight][bits];
        }
        let up = if tight == 1 { s[pos] - b'0' } else { 1 };
        let mut res = 0i64;
        for d in 0..=up {
            let nt = usize::from(tight == 1 && d == up);
            res = (res + dfs(pos + 1, nt, bits + d as usize, s, ops, k, memo)) % MOD;
        }
        memo[pos][tight][bits] = res;
        res
    }
    dfs(0, 1, 0, &s, &ops, k, &mut memo) as i32
}

fn main() {
    println!("{}", count_k_reducible_numbers("111".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::count_k_reducible_numbers;

    #[test]
    fn example1() {
        assert_eq!(count_k_reducible_numbers("111".into(), 1), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_k_reducible_numbers("1000".into(), 2), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(count_k_reducible_numbers("1".into(), 3), 0);
    }
}
