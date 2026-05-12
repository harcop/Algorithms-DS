/// LeetCode #686 - Repeated String Match
fn repeated_string_match(a: String, b: String) -> i32 {
    let mut s = String::new();
    let mut count = 0i32;
    while s.len() < b.len() { s.push_str(&a); count += 1; }
    if s.contains(&b) { return count; }
    s.push_str(&a);
    count += 1;
    if s.contains(&b) { return count; }
    -1
}

fn main() {
    println!("{}", repeated_string_match("abcd".into(), "cdabcdab".into()));
}

#[cfg(test)]
mod tests {
    use super::repeated_string_match;

    #[test]
    fn example_one() {
        assert_eq!(repeated_string_match("abcd".into(), "cdabcdab".into()), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(repeated_string_match("a".into(), "aa".into()), 2);
    }
}
