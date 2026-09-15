/// LeetCode #3747 - Count Distinct Integers After Removing Zeros
use std::collections::HashMap;

fn count_distinct(n: i64) -> i64 {
    let s: Vec<u8> = n.to_string().bytes().map(|c| c - b'0').collect();
    let mut memo: HashMap<(usize, u8, u8, u8), i64> = HashMap::new();
    dfs(0, 0, 1, 1, &s, &mut memo)
}

fn dfs(
    i: usize,
    zero: u8,
    lead: u8,
    limit: u8,
    s: &[u8],
    memo: &mut HashMap<(usize, u8, u8, u8), i64>,
) -> i64 {
    if i == s.len() {
        return if zero == 0 && lead == 0 { 1 } else { 0 };
    }
    let key = (i, zero, lead, limit);
    if limit == 0 {
        if let Some(&v) = memo.get(&key) {
            return v;
        }
    }
    let up = if limit == 1 { s[i] } else { 9 };
    let mut ans = 0i64;
    for d in 0..=up {
        let nxt_zero = if zero == 1 || (d == 0 && lead == 0) {
            1
        } else {
            0
        };
        let nxt_lead = if lead == 1 && d == 0 { 1 } else { 0 };
        let nxt_limit = if limit == 1 && d == up { 1 } else { 0 };
        ans += dfs(i + 1, nxt_zero, nxt_lead, nxt_limit, s, memo);
    }
    if limit == 0 {
        memo.insert(key, ans);
    }
    ans
}

fn main() {
    println!("{}", count_distinct(10));
}

#[cfg(test)]
mod tests {
    use super::count_distinct;

    #[test]
    fn example1() {
        assert_eq!(count_distinct(10), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(count_distinct(3), 3);
    }
}
