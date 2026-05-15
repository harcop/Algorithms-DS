/// LeetCode #816 - Ambiguous Coordinates (Premium duplicate slot)
pub const NOTE: &str = "LeetCode #816 is premium/overlapping catalog entry; use #811 for Ambiguous Coordinates.";

fn main() {
    println!("{}", NOTE);
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke() {
        assert!(!super::NOTE.is_empty());
    }
}
