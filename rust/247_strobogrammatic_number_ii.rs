/// LeetCode #247 - Strobogrammatic Number II
fn find_strobogrammatic(n: i32) -> Vec<String> {
    let pairs = [('0', '0'), ('1', '1'), ('6', '9'), ('8', '8'), ('9', '6')];
    let n = n as usize;
    let mut buf = vec![' '; n];
    let mut out = vec![];
    fn dfs(lo: usize, hi: usize, n_len: usize, buf: &mut [char], pairs: &[(char, char)], out: &mut Vec<String>) {
        if lo > hi {
            out.push(buf.iter().collect());
            return;
        }
        for &(a, b) in pairs {
            if lo == 0 && n_len > 1 && a == '0' {
                continue;
            }
            if lo == hi && !matches!(a, '0' | '1' | '8') {
                continue;
            }
            buf[lo] = a;
            buf[hi] = b;
            dfs(lo + 1, hi.saturating_sub(1), n_len, buf, pairs, out);
        }
    }
    dfs(0, n - 1, n, &mut buf, &pairs, &mut out);
    out
}

fn main() {
    println!("{:?}", find_strobogrammatic(2));
}

#[cfg(test)]
mod tests {
    use super::find_strobogrammatic;

    #[test]
    fn example_one() {
        let mut v = find_strobogrammatic(2);
        v.sort();
        assert_eq!(v, vec!["11", "69", "88", "96"]);
    }

    #[test]
    fn example_two() {
        let mut v = find_strobogrammatic(1);
        v.sort();
        assert_eq!(v, vec!["0", "1", "8"]);
    }
}
