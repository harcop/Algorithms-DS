/// LeetCode #3906 - Count Good Integers on a Grid Path
fn count_good_integers_on_path(l: i64, r: i64, directions: String) -> i64 {
    let mut key = [false; 16];
    let mut row = 0usize;
    let mut col = 0usize;
    key[0] = true;
    for c in directions.bytes() {
        if c == b'D' {
            row += 1;
        } else {
            col += 1;
        }
        key[row * 4 + col] = true;
    }
    calc(r, &key) - calc(l - 1, &key)
}

fn calc(x: i64, key: &[bool; 16]) -> i64 {
    if x < 0 {
        return 0;
    }
    let digits: Vec<u8> = format!("{x:016}").bytes().map(|b| b - b'0').collect();
    let mut memo = [[-1i64; 10]; 16];
    dfs(0, 0, true, &digits, key, &mut memo)
}

fn dfs(
    pos: usize,
    last: usize,
    lim: bool,
    digits: &[u8],
    key: &[bool; 16],
    memo: &mut [[i64; 10]; 16],
) -> i64 {
    if pos == 16 {
        return 1;
    }
    if !lim && memo[pos][last] != -1 {
        return memo[pos][last];
    }
    let start = if key[pos] { last } else { 0 };
    let end = if lim { digits[pos] as usize } else { 9 };
    let mut res = 0i64;
    if start <= end {
        for i in start..=end {
            let next_last = if key[pos] { i } else { last };
            res += dfs(pos + 1, next_last, lim && i == end, digits, key, memo);
        }
    }
    if !lim {
        memo[pos][last] = res;
    }
    res
}

fn main() {
    println!("{}", count_good_integers_on_path(8, 10, "DDDRRR".into()));
}

#[cfg(test)]
mod tests {
    use super::count_good_integers_on_path;

    #[test]
    fn example1() {
        assert_eq!(count_good_integers_on_path(8, 10, "DDDRRR".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_good_integers_on_path(123456789, 123456790, "DDRRDR".into()),
            1
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_good_integers_on_path(1288561398769758, 1288561398769758, "RRRDDD".into()),
            0
        );
    }
}
