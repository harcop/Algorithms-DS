/// LeetCode #1573 - Number Of Ways To Split A String
fn num_ways(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let b = s.as_bytes();
    let n = b.len();
    let ones: usize = b.iter().filter(|&&c| c == b'1').count();
    if ones == 0 {
        return ((n - 1) as i64 * (n - 2) as i64 / 2 % MOD) as i32;
    }
    if ones % 3 == 0 {
        let cnt = ones / 3;
        let mut pos = vec![];
        for (i, &c) in b.iter().enumerate() {
            if c == b'1' { pos.push(i); }
        }
        let i1 = pos[cnt - 1];
        let i2 = pos[cnt];
        let j1 = pos[2 * cnt - 1];
        let j2 = pos[2 * cnt];
        return (((i2 - i1) as i64 * (j2 - j1) as i64) % MOD) as i32;
    }
    let mut ans = 0i64;
    for i in 1..n {
        for j in i + 1..n {
            if b[..i].contains(&b'1') && b[i..j].contains(&b'1') && b[j..].contains(&b'1') {
                ans += 1;
            }
        }
    }
    (ans % MOD) as i32
}
fn main() { println!("{}", num_ways("1011".into())); }
#[cfg(test)]
mod tests {
    use super::num_ways;
    #[test]
    fn example_one() { assert_eq!(num_ways("1011".into()), 2); }
    #[test]
    fn example_two() { assert_eq!(num_ways("1001".into()), 0); }
    #[test]
    fn example_three() { assert_eq!(num_ways("0110101".into()), 8); }
}
