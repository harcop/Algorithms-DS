/// LeetCode #2843 - Count Symmetric Integers
fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    (low..=high)
        .filter(|&num| {
            let digits = num.to_string().into_bytes();
            if digits.len() % 2 != 0 {
                return false;
            }
            let half = digits.len() / 2;
            let left: u32 = digits[..half].iter().map(|&d| (d - b'0') as u32).sum();
            let right: u32 = digits[half..].iter().map(|&d| (d - b'0') as u32).sum();
            left == right
        })
        .count() as i32
}

fn main() {
    println!("{}", count_symmetric_integers(1, 100));
}

#[cfg(test)]
mod tests {
    use super::count_symmetric_integers;

    #[test]
    fn example_one() {
        assert_eq!(count_symmetric_integers(1, 100), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_symmetric_integers(1200, 1230), 4);
    }
}
