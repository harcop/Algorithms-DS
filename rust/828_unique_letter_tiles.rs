/// LeetCode #828 - Unique Letter Tiles (Premium)
pub const NOTE: &str = "LeetCode Premium #828 — Unique Letter Tiles; solve on LeetCode.";

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
