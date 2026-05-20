/// LeetCode #1165 - Single-Row Keyboard (Premium)
pub const NOTE: &str = "LeetCode Premium #1165 — solve on LeetCode.";

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
