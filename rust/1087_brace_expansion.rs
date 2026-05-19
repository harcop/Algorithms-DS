/// LeetCode #1087 - Brace Expansion (Premium)
pub const NOTE: &str = "LeetCode Premium #1087 — solve on LeetCode.";

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
