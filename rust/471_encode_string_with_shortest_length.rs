/// LeetCode #471 - Encode String with Shortest Length (premium; reference only)
pub const NOTE: &str = "Premium problem: DP over splits with repetition encoding.";

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
