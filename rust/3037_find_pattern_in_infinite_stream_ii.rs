/// LeetCode #3037 - Find Pattern in Infinite Stream II (Rust analogue)
fn build_lps(pat: &[i32]) -> Vec<usize> {
    let mut lps = vec![0usize; pat.len()];
    let mut len = 0usize;
    for i in 1..pat.len() {
        while len > 0 && pat[i] != pat[len] {
            len = lps[len - 1];
        }
        if pat[i] == pat[len] {
            len += 1;
        }
        lps[i] = len;
    }
    lps
}

fn find_pattern(stream: Vec<i32>, pattern: Vec<i32>) -> i32 {
    let n = stream.len();
    let m = pattern.len();
    if m == 0 || m > n {
        return -1;
    }
    let lps = build_lps(&pattern);
    let mut j = 0usize;
    for (i, &c) in stream.iter().enumerate() {
        while j > 0 && c != pattern[j] {
            j = lps[j - 1];
        }
        if c == pattern[j] {
            j += 1;
        }
        if j == m {
            return (i + 1 - m) as i32;
        }
    }
    -1
}

fn main() {
    let stream = vec![1, 1, 1, 0, 1, 1, 1];
    let pattern = vec![0, 1];
    println!("{}", find_pattern(stream, pattern));
}

#[cfg(test)]
mod tests {
    use super::find_pattern;

    #[test]
    fn example1() {
        assert_eq!(
            find_pattern(vec![1, 1, 1, 0, 1, 1, 1], vec![0, 1]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(find_pattern(vec![0, 0, 0, 0], vec![0]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(
            find_pattern(vec![1, 0, 1, 1, 0, 1, 1, 0, 1], vec![1, 1, 0, 1]),
            2
        );
    }
}
