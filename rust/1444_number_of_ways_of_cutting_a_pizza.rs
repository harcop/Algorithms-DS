/// LeetCode #1444 - Number Of Ways Of Cutting A Pizza
fn ways(pizza: Vec<String>, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let rows = pizza.len();
    let cols = pizza[0].len();
    let mut apple = vec![vec![0i32; cols + 1]; rows + 1];
    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            apple[i][j] = (pizza[i].as_bytes()[j] == b'A') as i32
                + apple[i + 1][j] + apple[i][j + 1] - apple[i + 1][j + 1];
        }
    }
    let get = |r1: usize, c1: usize, r2: usize, c2: usize| -> i32 {
        apple[r1][c1] - apple[r2 + 1][c1] - apple[r1][c2 + 1] + apple[r2 + 1][c2 + 1]
    };
    let cuts = k as usize - 1;
    let mut memo = vec![vec![vec![-1i64; cols]; rows]; cuts + 1];
    fn dp(rem: usize, r: usize, c: usize, rows: usize, cols: usize, get: &dyn Fn(usize, usize, usize, usize) -> i32, memo: &mut Vec<Vec<Vec<i64>>>) -> i64 {
        if get(r, c, rows - 1, cols - 1) == 0 { return 0; }
        if rem == 0 { return 1; }
        if memo[rem][r][c] != -1 { return memo[rem][r][c]; }
        let mut ans = 0i64;
        for nr in r + 1..rows {
            if get(r, c, nr - 1, cols - 1) > 0 { ans = (ans + dp(rem - 1, nr, c, rows, cols, get, memo)) % MOD; }
        }
        for nc in c + 1..cols {
            if get(r, c, rows - 1, nc - 1) > 0 { ans = (ans + dp(rem - 1, r, nc, rows, cols, get, memo)) % MOD; }
        }
        memo[rem][r][c] = ans;
        ans
    }
    dp(cuts, 0, 0, rows, cols, &get, &mut memo) as i32
}
fn main() { println!("{}", ways(vec!["A..".into(),"AAA".into(),"...".into()], 3)); }
#[cfg(test)]
mod tests {
    use super::ways;
    #[test]
    fn example_one() { assert_eq!(ways(vec!["A..".into(),"AAA".into(),"...".into()], 3), 3); }
    #[test]
    fn example_two() { assert_eq!(ways(vec!["A..".into(),"AA.".into(),"...".into()], 3), 1); }
}