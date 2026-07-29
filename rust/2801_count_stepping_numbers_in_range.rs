/// LeetCode #2801 - Count Stepping Numbers in Range
fn count_stepping_numbers(low: &str, high: &str) -> i32 {
    const MOD: i64 = 1_000_000_007;
    fn dfs(
        pos: usize,
        pre: i32,
        lead: bool,
        limit: bool,
        num: &[u8],
        memo: &mut [Option<i64>; 110 * 11],
    ) -> i64 {
        if pos >= num.len() {
            return if lead { 0 } else { 1 };
        }
        if !lead && !limit {
            let key = pos * 11 + (pre + 1) as usize;
            if let Some(v) = memo[key] {
                return v;
            }
        }
        let up = if limit {
            (num[pos] - b'0') as i32
        } else {
            9
        };
        let mut ans = 0i64;
        for i in 0..=up {
            if i == 0 && lead {
                ans += dfs(pos + 1, pre, true, limit && i == up, num, memo);
            } else if pre == -1 || (pre - i).abs() == 1 {
                ans += dfs(pos + 1, i, false, limit && i == up, num, memo);
            }
            ans %= MOD;
        }
        if !lead && !limit {
            let key = pos * 11 + (pre + 1) as usize;
            memo[key] = Some(ans);
        }
        ans
    }
    fn count(num: &str) -> i64 {
        let bytes = num.as_bytes();
        let mut memo = [None; 110 * 11];
        dfs(0, -1, true, true, bytes, &mut memo)
    }
    let mut low_minus_one = low.to_string();
    let bytes = unsafe { low_minus_one.as_bytes_mut() };
    for i in (0..bytes.len()).rev() {
        if bytes[i] != b'0' {
            bytes[i] -= 1;
            break;
        }
        bytes[i] = b'9';
    }
    let a = count(high);
    let b = count(std::str::from_utf8(bytes).unwrap());
    ((a - b + MOD) % MOD) as i32
}

fn main() {
    println!("{}", count_stepping_numbers("1", "11"));
}

#[cfg(test)]
mod tests {
    use super::count_stepping_numbers;

    #[test]
    fn example_one() {
        assert_eq!(count_stepping_numbers("1", "11"), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_stepping_numbers("90", "101"), 2);
    }
}
