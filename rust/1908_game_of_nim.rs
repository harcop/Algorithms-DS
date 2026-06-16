/// LeetCode #1908 - Game of Nim
fn nim_game(piles: Vec<i32>) -> bool {
    piles.iter().fold(0, |acc, &x| acc ^ x) != 0
}

fn main() {
    println!("{}", nim_game(vec![1]));
}

#[cfg(test)]
mod tests {
    use super::nim_game;

    #[test]
    fn example_one() {
        assert!(nim_game(vec![1]));
    }

    #[test]
    fn example_two() {
        assert!(!nim_game(vec![1, 1]));
    }

    #[test]
    fn example_three() {
        assert!(nim_game(vec![1, 2, 3, 4]));
    }
}
