/// LeetCode #2160 - Minimum Sum of Four Digit Number After Splitting Digits
fn minimum_sum(num: i32) -> i32 {
    let mut digits = Vec::new();
    let mut x = num;
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    digits.sort_unstable();
    digits[0] * 10 + digits[2] + digits[1] * 10 + digits[3]
}

fn main() {
    println!("{}", minimum_sum(2932));
}

#[cfg(test)]
mod tests {
    use super::minimum_sum;

    #[test]
    fn example_one() {
        assert_eq!(minimum_sum(2932), 52);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_sum(4009), 13);
    }
}
