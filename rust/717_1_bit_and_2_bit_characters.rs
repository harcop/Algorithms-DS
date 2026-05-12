/// LeetCode #717 - 1-bit and 2-bit Characters
fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let n = bits.len();
    let mut i = 0usize;
    while i < n - 1 {
        i += if bits[i] == 1 { 2 } else { 1 };
    }
    i == n - 1
}

fn main() {
    println!("{}", is_one_bit_character(vec![1,0,0]));
}

#[cfg(test)]
mod tests {
    use super::is_one_bit_character;

    #[test]
    fn example_one() {
        assert!(is_one_bit_character(vec![1,0,0]));
    }

    #[test]
    fn example_two() {
        assert!(!is_one_bit_character(vec![1,1,1,0]));
    }
}
