/// LeetCode #1556 - Thousand Separator
fn thousand_separator(n: i32) -> String {
    let s = n.to_string();
    let mut ans = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            ans.push('.');
        }
        ans.push(c);
    }
    ans.chars().rev().collect()
}

fn main() {
    println!("{}", thousand_separator(987));
}

#[cfg(test)]
mod tests {
    use super::thousand_separator;

    #[test]
    fn example_one() {
        assert_eq!(thousand_separator(987), "987");
    }

    #[test]
    fn example_two() {
        assert_eq!(thousand_separator(1234), "1.234");
    }
}
