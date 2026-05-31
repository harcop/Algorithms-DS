/// LeetCode #1581 - Customer Who Visited But Did Not Make Any Transactions
pub const NOTE: &str = "SQL problem; omitted in this set.";
fn main() { println!("{}", NOTE.len()); }
#[cfg(test)]
mod tests { use super::NOTE; #[test] fn note_non_empty() { assert!(!NOTE.is_empty()); } }