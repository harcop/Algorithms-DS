/// LeetCode #3174 - Clear Digits
fn clear_digits(s: String) -> String {
    let mut stk = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            stk.pop();
        } else {
            stk.push(c);
        }
    }
    stk
}

fn main() {
    println!("{}", clear_digits("cb34".into()));
}

#[cfg(test)]
mod tests {
    use super::clear_digits;

    #[test]
    fn example1() {
        assert_eq!(clear_digits("abc".into()), "abc");
    }

    #[test]
    fn example2() {
        assert_eq!(clear_digits("cb34".into()), "");
    }
}
