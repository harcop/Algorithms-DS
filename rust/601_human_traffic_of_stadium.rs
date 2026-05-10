/// LeetCode #601 - Human Traffic Of Stadium (SQL; reference only)
pub const NOTE: &str = "SQL problem; omitted in this set.";

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
