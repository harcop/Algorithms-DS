/// LeetCode #709 - To Lower Case
fn to_lower_case(s: String) -> String {
    s.chars().map(|c| c.to_ascii_lowercase()).collect()
}

fn main() {
    println!("{}", to_lower_case("Hello".into()));
}

#[cfg(test)]
mod tests {
    use super::to_lower_case;

    #[test]
    fn example_one() {
        assert_eq!(to_lower_case("Hello".into()), "hello");
    }
}
