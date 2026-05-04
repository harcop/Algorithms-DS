/// LeetCode #294 - Median of Unsorted Arrays (premium; reference only)
pub const NOTE: &str = "Premium problem: use selection / quickselect on two unsorted arrays.";

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
