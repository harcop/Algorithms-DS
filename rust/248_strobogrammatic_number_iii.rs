/// LeetCode #248 - Strobogrammatic Number III
fn strobogrammatic_in_range(low: String, high: String) -> i32 {
    let pairs = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
    let lo_len = low.len();
    let hi_len = high.len();
    let mut ans = 0i32;

    for len in lo_len..=hi_len {
        let mut buf = vec![' '; len];
        let mut all = Vec::new();
        fn dfs(lo_i: usize, hi_i: usize, buf: &mut [char], pairs: &[(char, char)], out: &mut Vec<String>) {
            if lo_i > hi_i {
                out.push(buf.iter().collect());
                return;
            }
            for &(a, b) in pairs {
                if lo_i == 0 && buf.len() > 1 && a == '0' {
                    continue;
                }
                if lo_i == hi_i && !matches!(a, '0' | '1' | '8') {
                    continue;
                }
                buf[lo_i] = a;
                buf[hi_i] = b;
                dfs(lo_i + 1, hi_i.saturating_sub(1), buf, pairs, out);
            }
        }
        dfs(0, len - 1, &mut buf, &pairs, &mut all);
        for s in all {
            let ok_low = len > lo_len || s >= low;
            let ok_high = len < hi_len || s <= high;
            if ok_low && ok_high {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", strobogrammatic_in_range("50".into(), "100".into()));
}

#[cfg(test)]
mod tests {
    use super::strobogrammatic_in_range;

    #[test]
    fn example_one() {
        assert_eq!(strobogrammatic_in_range("50".into(), "100".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(strobogrammatic_in_range("0".into(), "0".into()), 1);
    }
}
