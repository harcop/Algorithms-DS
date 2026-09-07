/// LeetCode #3612 - Process String with Special Operations I
fn process_str(s: String) -> String {
    let mut result = Vec::new();
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            result.push(c);
        } else if c == '*' {
            result.pop();
        } else if c == '#' {
            result.extend_from_within(..);
        } else if c == '%' {
            result.reverse();
        }
    }
    result.into_iter().collect()
}

fn main() {
    println!("{}", process_str("a#b%*".into()));
}

#[cfg(test)]
mod tests {
    use super::process_str;

    #[test]
    fn example1() {
        assert_eq!(process_str("a#b%*".into()), "ba");
    }

    #[test]
    fn example2() {
        assert_eq!(process_str("z*#".into()), "");
    }
}
