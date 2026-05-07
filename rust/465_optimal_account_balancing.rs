/// LeetCode #465 - Optimal Account Balancing (premium; reference only)
pub const NOTE: &str = "Premium problem: min cash flow / subset-sum style balancing.";

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
