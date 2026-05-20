/// LeetCode #1134 - Armstrong Number (Premium)
pub const NOTE: &str = "LeetCode Premium #1134 — solve on LeetCode.";

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
