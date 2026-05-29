/// LeetCode #1541 - Minimum Insertions To Balance A Parentheses String
fn min_insertions(s: String) -> i32 {
    let mut open = 0i32;
    let mut ans = 0i32;
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'(' {
            open += 1;
            i += 1;
        } else {
            if i + 1 < bytes.len() && bytes[i + 1] == b')' {
                i += 2;
            } else {
                ans += 1;
                i += 1;
            }
            if open == 0 {
                ans += 1;
            } else {
                open -= 1;
            }
        }
    }
    ans + open * 2
}

fn main() {
    println!("{}", min_insertions("(()))".into()));
}

#[cfg(test)]
mod tests {
    use super::min_insertions;

    #[test]
    fn example_one() {
        assert_eq!(min_insertions("(()))".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_insertions("())".into()), 0);
    }
}
