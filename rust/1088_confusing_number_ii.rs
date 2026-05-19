/// LeetCode #1088 - Confusing Number II (Premium)
pub const NOTE: &str = "LeetCode Premium #1088 — solve on LeetCode.";

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
