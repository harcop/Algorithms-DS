/// LeetCode #1135 - Corporate Flight Bookings II (Premium)
pub const NOTE: &str = "LeetCode Premium #1135 — solve on LeetCode.";

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
