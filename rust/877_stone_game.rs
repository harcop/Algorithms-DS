/// LeetCode #877 - Stone Game
fn stone_game(_piles: Vec<i32>) -> bool {
    true
}

fn main() {
    println!("{}", stone_game(vec![5, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::stone_game;

    #[test]
    fn example_one() {
        assert!(stone_game(vec![5, 3, 4, 5]));
    }
}
