/// LeetCode #3798 - Largest Even Number
fn largest_even(s: String) -> String {
    let t = s.trim_end_matches('1');
    t.to_string()
}

fn main() {
    println!("{}", largest_even("1112".into()));
}

#[cfg(test)]
mod tests {
    use super::largest_even;

    #[test]
    fn example1() {
        assert_eq!(largest_even("1112".into()), "1112");
    }

    #[test]
    fn example2() {
        assert_eq!(largest_even("221".into()), "22");
    }

    #[test]
    fn example3() {
        assert_eq!(largest_even("1".into()), "");
    }
}
