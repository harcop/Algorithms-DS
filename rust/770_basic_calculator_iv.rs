/// LeetCode #770 - Basic Calculator IV (very large expression + evalmap surface)
pub const NOTE: &str = "LeetCode #770 — polynomial-style calculator with evalmap; full parser omitted in-repo.";

fn main() {
    println!("{}", NOTE.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        assert!(!super::NOTE.is_empty());
    }
}
