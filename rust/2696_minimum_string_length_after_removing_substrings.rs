/// LeetCode #2696 - Minimum String Length After Removing Substrings
fn min_length(s: String) -> i32 {
    let mut stk: Vec<u8> = Vec::new();
    for c in s.bytes() {
        if let Some(&last) = stk.last() {
            if (last == b'A' && c == b'B') || (last == b'C' && c == b'D') {
                stk.pop();
                continue;
            }
        }
        stk.push(c);
    }
    stk.len() as i32
}

fn main() {
    println!("{}", min_length("ABFCACDB".into()));
}

#[cfg(test)]
mod tests {
    use super::min_length;

    #[test]
    fn example_one() {
        assert_eq!(min_length("ABFCACDB".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_length("ACBBD".into()), 5);
    }
}
