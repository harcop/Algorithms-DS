/// LeetCode #1168 - Currency Converter (Premium)
pub const NOTE: &str = "LeetCode Premium #1168 — solve on LeetCode.";

fn main() {
    println!("{}", NOTE);
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        assert!(!super::NOTE.is_empty());
    }
}
