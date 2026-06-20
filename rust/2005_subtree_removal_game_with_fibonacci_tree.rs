/// LeetCode #2005 - Subtree Removal Game with Fibonacci Tree
fn find_game_winner(n: i32) -> bool {
    n % 6 != 1
}

fn main() {
    println!("{}", find_game_winner(3));
}

#[cfg(test)]
mod tests {
    use super::find_game_winner;

    #[test]
    fn example_one() {
        assert!(find_game_winner(3));
    }

    #[test]
    fn example_two() {
        assert!(!find_game_winner(1));
    }

    #[test]
    fn example_three() {
        assert!(find_game_winner(2));
    }
}
