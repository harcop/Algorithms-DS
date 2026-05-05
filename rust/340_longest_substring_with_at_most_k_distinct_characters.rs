/// LeetCode #340 - Longest Substring With At Most K Distinct Characters (premium; reference only)
pub const NOTE: &str = "Premium problem: implementation omitted in this set.";

fn main() {
    println!("{}", NOTE.len());
}

#[cfg(test)]
mod tests {
    use super::NOTE;

    #[test]
    fn note_non_empty() {
        assert!(!NOTE.is_empty());
    }
}
