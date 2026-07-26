/// LeetCode #2683 - Neighboring Bitwise XOR
fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    derived.iter().fold(0, |acc, &x| acc ^ x) == 0
}

fn main() {
    println!("{}", does_valid_array_exist(vec![1, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::does_valid_array_exist;

    #[test]
    fn example_one() {
        assert!(does_valid_array_exist(vec![1, 1, 0]));
    }

    #[test]
    fn example_two() {
        assert!(does_valid_array_exist(vec![1, 1]));
    }

    #[test]
    fn example_three() {
        assert!(!does_valid_array_exist(vec![1, 0]));
    }
}
