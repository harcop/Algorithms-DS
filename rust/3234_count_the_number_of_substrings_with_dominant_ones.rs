/// LeetCode #3234 - Count the Number of Substrings With Dominant Ones
fn number_of_substrings(s: String) -> i32 {
    let cs = s.as_bytes();
    let n = cs.len();
    let mut nxt = vec![n; n + 1];
    for i in (0..n).rev() {
        nxt[i] = nxt[i + 1];
        if cs[i] == b'0' {
            nxt[i] = i;
        }
    }
    let mut ans = 0i64;
    for i in 0..n {
        let mut cnt0 = if cs[i] == b'0' { 1usize } else { 0 };
        let mut j = i;
        while j < n && (cnt0 as i64) * (cnt0 as i64) <= n as i64 {
            let cnt1 = nxt[j + 1] - i - cnt0;
            if cnt1 >= cnt0 * cnt0 {
                ans += (nxt[j + 1] - j).min(cnt1 - cnt0 * cnt0 + 1) as i64;
            }
            j = nxt[j + 1];
            cnt0 += 1;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", number_of_substrings("00011".into()));
}

#[cfg(test)]
mod tests {
    use super::number_of_substrings;

    #[test]
    fn example1() {
        assert_eq!(number_of_substrings("00011".into()), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_substrings("101101".into()), 16);
    }
}
