/// LeetCode #693 - Binary Number with Alternating Bits
fn has_alternating_bits(n: i32) -> bool {
    let x = (n ^ (n >> 1)) as i64;
    (x & (x + 1)) == 0
}

fn main() {
    println!("{}", has_alternating_bits(5));
}

#[cfg(test)]
mod tests {
    use super::has_alternating_bits;

    #[test]
    fn example_one() {
        assert!(has_alternating_bits(5));
    }

    #[test]
    fn example_two() {
        assert!(!has_alternating_bits(7));
    }
}
