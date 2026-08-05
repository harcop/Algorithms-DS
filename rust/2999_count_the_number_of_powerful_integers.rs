/// LeetCode #2999 - Count the Number of Powerful Integers
use std::collections::HashMap;

fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
    fn count_up_to(t: &str, n: usize, suffix: &str, limit: i32, memo: &mut HashMap<(usize, bool), i64>) -> i64 {
        fn dfs(
            pos: usize,
            lim: bool,
            t: &str,
            n: usize,
            suffix: &str,
            limit: i32,
            memo: &mut HashMap<(usize, bool), i64>,
        ) -> i64 {
            if t.len() < n {
                return 0;
            }
            if t.len() - pos == n {
                let ok = if lim {
                    suffix <= &t[pos..]
                } else {
                    true
                };
                return if ok { 1 } else { 0 };
            }
            if let Some(&ans) = memo.get(&(pos, lim)) {
                return ans;
            }
            let up = (if lim {
                t.as_bytes()[pos] - b'0'
            } else {
                9
            } as i32)
                .min(limit);
            let mut ans = 0i64;
            for i in 0..=up {
                let next_lim = lim && i == (t.as_bytes()[pos] - b'0') as i32;
                ans += dfs(pos + 1, next_lim, t, n, suffix, limit, memo);
            }
            memo.insert((pos, lim), ans);
            ans
        }
        dfs(0, true, t, n, suffix, limit, memo)
    }

    let n = s.len();
    let t_start = (start - 1).to_string();
    let mut memo = HashMap::new();
    let a = count_up_to(&t_start, n, &s, limit, &mut memo);
    memo.clear();
    let t_finish = finish.to_string();
    let b = count_up_to(&t_finish, n, &s, limit, &mut memo);
    b - a
}

fn main() {
    println!("{}", number_of_powerful_int(1, 6000, 4, "124".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_powerful_int;

    #[test]
    fn example_one() {
        assert_eq!(number_of_powerful_int(1, 6000, 4, "124".into()), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(number_of_powerful_int(15, 215, 6, "10".into()), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(number_of_powerful_int(1000, 2000, 4, "3000".into()), 0);
    }
}
