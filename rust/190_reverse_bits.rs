/// LeetCode #190 - Reverse Bits
fn reverse_bits(mut x: u32) -> u32 {
    let mut y = 0u32;
    for _ in 0..32 {
        y = (y << 1) | (x & 1);
        x >>= 1;
    }
    y
}

fn main() {
    println!("{}", reverse_bits(43261596));
}

#[cfg(test)]
mod tests {
    use super::reverse_bits;

    #[test]
    fn example_one() {
        assert_eq!(reverse_bits(43261596), 964176192);
    }

    #[test]
    fn example_two() {
        assert_eq!(reverse_bits(4294967293), 3221225471);
    }
}
