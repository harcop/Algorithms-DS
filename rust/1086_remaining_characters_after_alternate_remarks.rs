/// LeetCode #1086 - Remaining Characters After Alternate Remarks (Premium)
pub const NOTE: &str = "LeetCode Premium #1086 — solve on LeetCode.";

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
