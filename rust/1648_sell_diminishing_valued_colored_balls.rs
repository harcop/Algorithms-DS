/// LeetCode #1648 - Sell Diminishing Valued Colored Balls
fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut inv = inventory;
    inv.sort_unstable_by(|a, b| b.cmp(a));
    let mut rem = orders as i64;
    let mut ans = 0i64;
    let mut i = 0usize;
    while i < inv.len() && rem > 0 {
        let j = (i + 1..=inv.len()).find(|&j| j == inv.len() || inv[j] < inv[i]).unwrap();
        let cnt = (j - i) as i64;
        let hi = inv[i] as i64;
        let lo = if j == inv.len() { 0 } else { inv[j] as i64 };
        let span = hi - lo;
        let take = rem.min(cnt * span);
        let full = take / cnt;
        let rest = take % cnt;
        ans = (ans + cnt * full * (2 * hi - full + 1) / 2) % MOD;
        ans = (ans + rest * (2 * hi - full - 2 * rest + 1) / 2) % MOD;
        rem -= take;
        i = j;
    }
    ans as i32
}
fn main() { println!("{}", max_profit(vec![2,5], 4)); }
#[cfg(test)]
mod tests {
    use super::max_profit;
    #[test]
    fn example_one() { assert_eq!(max_profit(vec![2,5], 4), 14); }
}