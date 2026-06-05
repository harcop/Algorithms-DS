/// LeetCode #1731 - The Number Of Employees Which Report To Each Employee
pub const NOTE: &str = "SQL problem; omitted in this set.";
fn main() { println!("{}", NOTE.len()); }
#[cfg(test)]
mod tests {
    use super::NOTE;
    #[test]
    fn note_non_empty() {
        assert!(!NOTE.is_empty());
    }
}
