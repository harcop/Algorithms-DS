/// LeetCode #1236 - Web Crawler (Premium)
pub const NOTE: &str = "LeetCode Premium #1236 — solve on LeetCode.";

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
