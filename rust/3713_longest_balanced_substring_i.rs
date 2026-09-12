/// LeetCode #3713 - Longest Balanced Substring I
fn longest_balanced(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut result = 0;
    for i in 0..n {
        let mut cnt = [0i32; 26];
        let mut mx = 0;
        let mut unique = 0;
        for j in i..n {
            let idx = (bytes[j] - b'a') as usize;
            if cnt[idx] == 0 {
                unique += 1;
            }
            cnt[idx] += 1;
            mx = mx.max(cnt[idx]);
            let len = (j - i + 1) as i32;
            if len % unique == 0 && len / unique == mx {
                result = result.max(len);
            }
        }
    }
    result
}

fn main() {
    println!("{}", longest_balanced("abbac".into()));
}

#[cfg(test)]
mod tests {
    use super::longest_balanced;

    #[test]
    fn example1() {
        assert_eq!(longest_balanced("abbac".into()), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_balanced("zzabccy".into()), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_balanced("aba".into()), 2);
    }
}
