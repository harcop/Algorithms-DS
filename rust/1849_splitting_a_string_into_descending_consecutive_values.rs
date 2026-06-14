/// LeetCode #1849 - Splitting a String Into Descending Consecutive Values
fn split_string(s: String) -> bool {
    let b = s.as_bytes();

    fn dfs(b: &[u8], i: usize, x: i64) -> bool {
        if i >= b.len() {
            return true;
        }
        let mut y = 0i64;
        let end = if x < 0 { b.len() - 1 } else { b.len() };
        for j in i..end {
            y = y * 10 + (b[j] - b'0') as i64;
            if (x < 0 || x - y == 1) && dfs(b, j + 1, y) {
                return true;
            }
        }
        false
    }

    dfs(b, 0, -1)
}

fn main() {
    println!("{}", split_string("050043".into()));
}

#[cfg(test)]
mod tests {
    use super::split_string;

    #[test]
    fn example_one() {
        assert!(!split_string("1234".into()));
    }

    #[test]
    fn example_two() {
        assert!(split_string("050043".into()));
    }

    #[test]
    fn example_three() {
        assert!(!split_string("9080701".into()));
    }
}
