/// LeetCode #2719 - Count of Integers
fn count(num1: String, num2: String, min_sum: i32, max_sum: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    fn dfs(
        num: &[u8],
        pos: usize,
        s: i32,
        limit: bool,
        min_sum: i32,
        max_sum: i32,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if pos >= num.len() {
            return if s >= min_sum && s <= max_sum { 1 } else { 0 };
        }
        if !limit && memo[pos][s as usize] != -1 {
            return memo[pos][s as usize];
        }
        let up = if limit { (num[pos] - b'0') as i32 } else { 9 };
        let mut ans = 0i32;
        for i in 0..=up {
            ans = (ans
                + dfs(
                    num,
                    pos + 1,
                    s + i,
                    limit && i == up,
                    min_sum,
                    max_sum,
                    memo,
                ))
                % MOD;
        }
        if !limit {
            memo[pos][s as usize] = ans;
        }
        ans
    }

    fn calc(num: &str, min_sum: i32, max_sum: i32) -> i32 {
        let bytes = num.as_bytes();
        let mut memo = vec![vec![-1; 220]; bytes.len() + 1];
        dfs(bytes, 0, 0, true, min_sum, max_sum, &mut memo)
    }

    fn dec(num: &str) -> String {
        let mut t: Vec<u8> = num.as_bytes().to_vec();
        for i in (0..t.len()).rev() {
            if t[i] != b'0' {
                t[i] -= 1;
                break;
            }
            t[i] = b'9';
        }
        // strip leading zeros but keep at least one digit
        let mut start = 0;
        while start + 1 < t.len() && t[start] == b'0' {
            start += 1;
        }
        String::from_utf8(t[start..].to_vec()).unwrap()
    }

    let a = calc(&num2, min_sum, max_sum);
    let b = calc(&dec(&num1), min_sum, max_sum);
    (a - b + MOD) % MOD
}

fn main() {
    println!("{}", count("1".into(), "12".into(), 1, 8));
}

#[cfg(test)]
mod tests {
    use super::count;

    #[test]
    fn example_one() {
        assert_eq!(count("1".into(), "12".into(), 1, 8), 11);
    }

    #[test]
    fn example_two() {
        assert_eq!(count("1".into(), "5".into(), 1, 5), 5);
    }
}
