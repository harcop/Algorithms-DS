/// LeetCode #1166 - Design File System (Premium)
pub const NOTE: &str = "LeetCode Premium #1166 — solve on LeetCode.";

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
