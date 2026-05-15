/// LeetCode #810 - Chalkboard XOR Game
fn xor_game(nums: Vec<i32>) -> bool {
    nums.iter().fold(0, |a, &b| a ^ b) == 0
}

fn main() {
    println!("{}", xor_game(vec![1, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::xor_game;

    #[test]
    fn example_one() {
        assert!(!xor_game(vec![1, 1, 2]));
    }

    #[test]
    fn example_two() {
        assert!(!xor_game(vec![0, 1]));
    }
}
