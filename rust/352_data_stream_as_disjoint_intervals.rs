/// LeetCode #352 - Data Stream As Disjoint Intervals (premium; reference only)
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
