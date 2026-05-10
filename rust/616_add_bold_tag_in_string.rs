/// LeetCode #616 - Add Bold Tag In String (premium; reference only)
pub const NOTE: &str = "Premium problem; omitted in this set.";

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
