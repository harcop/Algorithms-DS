/// LeetCode #4060 - Count Evenly Good Integers
fn count_evenly_good_integers(l: i64, r: i64) -> i64 {
    calc(r) - calc(l - 1)
}

fn calc(x: i64) -> i64 {
    let s: Vec<u8> = x.to_string().into_bytes();
    let mut memo = vec![[[-1i64; 2]; 2]; s.len()];
    fn dfs(s: &[u8], memo: &mut [[[i64; 2]; 2]], pos: usize, st: usize, lim: bool) -> i64 {
        if pos == s.len() {
            return (st ^ 1) as i64;
        }
        let k = usize::from(lim);
        if memo[pos][st][k] != -1 {
            return memo[pos][st][k];
        }
        let up = if lim { (s[pos] - b'0') as usize } else { 9 };
        let mut res = 0i64;
        for i in 0..=up {
            let nxt = (st + ((i & 1) ^ 1)) % 2;
            res += dfs(s, memo, pos + 1, nxt, lim && i == up);
        }
        memo[pos][st][k] = res;
        res
    }
    dfs(&s, &mut memo, 0, 0, true)
}

fn main() {
    println!("{}", count_evenly_good_integers(18, 22));
}

#[cfg(test)]
mod tests {
    use super::count_evenly_good_integers;

    #[test]
    fn example1() {
        assert_eq!(count_evenly_good_integers(18, 22), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_evenly_good_integers(98, 101), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(count_evenly_good_integers(1, 10), 5);
    }
}
