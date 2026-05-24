/// LeetCode #1242 - Web Crawler Multithreaded (Premium)
pub const NOTE: &str = "LeetCode Premium #1242 — solve on LeetCode.";

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
