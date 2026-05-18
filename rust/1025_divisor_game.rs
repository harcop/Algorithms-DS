/// LeetCode #1025 - Divisor Game
fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("{}", divisor_game(2));
}

#[cfg(test)]
mod tests {
    use super::motion_game;

    #[test]
    fn example_one() {
        assert!(divisor_game(2));
    }

    #[test]
    fn example_two() {
        assert!(!divisor_game(3));
    }
}
