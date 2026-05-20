/// LeetCode #1153 - String Transforms Into Another String (Premium)
pub const NOTE: &str = "LeetCode Premium #1153 — solve on LeetCode.";

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
