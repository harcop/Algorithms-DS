/// LeetCode #2429 - Minimize XOR
fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let mut bits_needed = num2.count_ones();
    let mut answer = 0;

    for bit in (0..31).rev() {
        let mask = 1 << bit;
        if num1 & mask != 0 && bits_needed > 0 {
            answer |= mask;
            bits_needed -= 1;
        }
    }

    for bit in 0..31 {
        let mask = 1 << bit;
        if answer & mask == 0 && bits_needed > 0 {
            answer |= mask;
            bits_needed -= 1;
        }
    }

    answer
}

fn main() {
    println!("{}", minimize_xor(3, 5));
}

#[cfg(test)]
mod tests {
    use super::minimize_xor;

    #[test]
    fn example_one() {
        assert_eq!(minimize_xor(3, 5), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimize_xor(1, 12), 3);
    }
}
