/// LeetCode #2980 - Check if Bitwise OR Has Trailing Zeros
fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    nums.iter().filter(|&&x| x & 1 == 0).count() >= 2
}

fn main() {
    println!("{}", has_trailing_zeros(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::has_trailing_zeros;

    #[test]
    fn example_one() {
        assert!(has_trailing_zeros(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn example_two() {
        assert!(has_trailing_zeros(vec![2, 4, 8, 16]));
    }

    #[test]
    fn example_three() {
        assert!(!has_trailing_zeros(vec![1, 3, 5, 7, 9]));
    }
}
