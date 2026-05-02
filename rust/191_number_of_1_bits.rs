/// LeetCode #191 - Number of 1 Bits
fn hamming_weight(mut n: u32) -> i32 {
    let mut c = 0;
    while n != 0 {
        n &= n - 1;
        c += 1;
    }
    c
}

fn main() {
    println!("{}", hamming_weight(11));
}

#[cfg(test)]
mod tests {
    use super::hamming_weight;

    #[test]
    fn example_one() {
        assert_eq!(hamming_weight(11), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(hamming_weight(128), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(hamming_weight(2147483645), 30);
    }
}
