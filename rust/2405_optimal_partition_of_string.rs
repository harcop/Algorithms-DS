/// LeetCode #2405 - Optimal Partition of String
fn partition_string(s: String) -> i32 {
    let mut seen = 0u32;
    let mut parts = 1;

    for b in s.bytes() {
        let bit = 1u32 << (b - b'a');
        if seen & bit != 0 {
            parts += 1;
            seen = 0;
        }
        seen |= bit;
    }

    parts
}

fn main() {
    println!("{}", partition_string("abacaba".to_string()));
}

#[cfg(test)]
mod tests {
    use super::partition_string;

    #[test]
    fn example_one() {
        assert_eq!(partition_string("abacaba".to_string()), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(partition_string("ssssss".to_string()), 6);
    }
}
