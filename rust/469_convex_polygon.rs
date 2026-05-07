/// LeetCode #469 - Convex Polygon (premium; reference only)
pub const NOTE: &str = "Premium problem: convex hull / cross-product orientation checks.";

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
