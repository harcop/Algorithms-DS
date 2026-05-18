/// LeetCode #902 - Numbers At Most N Given Digit Set
fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
    let mut ds: Vec<char> = digits.iter().flat_map(|d| d.chars()).collect();
    ds.sort_unstable();
    ds.dedup();
    let s: Vec<char> = n.to_string().chars().collect();
    let k = s.len();
    let m = ds.len();

    fn pow_m(base: usize, exp: usize) -> i64 {
        let mut r = 1i64;
        for _ in 0..exp {
            r *= base as i64;
        }
        r
    }

    let count_len = |len: usize| -> i64 {
        if len == 0 {
            return 0;
        }
        let has0 = ds.contains(&'0');
        if !has0 {
            return pow_m(m, len);
        }
        if len == 1 {
            return ds.iter().filter(|&&c| c != '0').count() as i64;
        }
        let nz = ds.iter().filter(|&&c| c != '0').count() as i64;
        nz * pow_m(m, len - 1)
    };

    let mut ans = 0i64;
    for len in 1..k {
        ans += count_len(len);
    }

    fn dfs(
        pos: usize,
        started: bool,
        tight: bool,
        s: &[char],
        ds: &[char],
    ) -> i64 {
        if pos == s.len() {
            return if started { 1 } else { 0 };
        }
        let lim = if tight { s[pos] } else { '9' };
        let mut res = 0i64;
        if !started {
            for &d in ds {
                if d == '0' {
                    continue;
                }
                if d > lim {
                    break;
                }
                let nt = tight && d == lim;
                res += dfs(pos + 1, true, nt, s, ds);
            }
        } else {
            for &d in ds {
                if d > lim {
                    break;
                }
                let nt = tight && d == lim;
                res += dfs(pos + 1, true, nt, s, ds);
            }
        }
        res
    }

    ans += dfs(0, false, true, &s, &ds);
    ans as i32
}

fn main() {
    println!(
        "{}",
        at_most_n_given_digit_set(vec!["1".into(), "3".into(), "5".into(), "7".into()], 100)
    );
}

#[cfg(test)]
mod tests {
    use super::at_most_n_given_digit_set;

    #[test]
    fn example_one() {
        let d = vec!["1", "3", "5", "7"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(at_most_n_given_digit_set(d, 100), 20);
    }
}
