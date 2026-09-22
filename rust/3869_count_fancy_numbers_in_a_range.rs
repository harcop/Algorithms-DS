/// LeetCode #3869 - Count Fancy Numbers in a Range
use std::collections::HashMap;

fn check(s: i32) -> bool {
    if s < 100 {
        s % 11 != 0
    } else {
        let mid = (s / 10) % 10;
        let last = s % 10;
        mid > 1 && mid < last
    }
}

fn dfs(
    pos: usize,
    s: i32,
    prev: i32,
    st: i32,
    lim: bool,
    num: &[u8],
    memo: &mut HashMap<(usize, i32, i32, i32), i64>,
) -> i64 {
    if pos >= num.len() {
        return if st != 3 {
            1
        } else if check(s) {
            1
        } else {
            0
        };
    }
    if !lim {
        if let Some(&v) = memo.get(&(pos, s, prev, st)) {
            return v;
        }
    }
    let up = if lim { (num[pos] - b'0') as i32 } else { 9 };
    let mut res = 0i64;
    for i in 0..=up {
        let nxt_st = match st {
            0 => {
                if prev == 0 {
                    0
                } else if i > prev {
                    1
                } else if i < prev {
                    2
                } else {
                    3
                }
            }
            1 => {
                if i > prev {
                    1
                } else {
                    3
                }
            }
            2 => {
                if i < prev {
                    2
                } else {
                    3
                }
            }
            _ => 3,
        };
        res += dfs(pos + 1, s + i, i, nxt_st, lim && i == up, num, memo);
    }
    if !lim {
        memo.insert((pos, s, prev, st), res);
    }
    res
}

fn calc(x: i64) -> i64 {
    let num = x.to_string().into_bytes();
    let mut memo = HashMap::new();
    dfs(0, 0, 0, 0, true, &num, &mut memo)
}

fn count_fancy(l: i64, r: i64) -> i64 {
    calc(r) - calc(l - 1)
}

fn main() {
    println!("{}", count_fancy(8, 10));
}

#[cfg(test)]
mod tests {
    use super::count_fancy;

    #[test]
    fn example1() {
        assert_eq!(count_fancy(8, 10), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_fancy(12340, 12341), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(count_fancy(123456788, 123456788), 0);
    }
}
