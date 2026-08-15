/// LeetCode #3222 - Find the Winning Player in Coin Game
fn losing_player(x: i32, y: i32) -> String {
    if x.min(y / 4) % 2 == 1 {
        "Alice".into()
    } else {
        "Bob".into()
    }
}

fn main() {
    println!("{}", losing_player(2, 7));
}

#[cfg(test)]
mod tests {
    use super::losing_player;

    #[test]
    fn example1() {
        assert_eq!(losing_player(2, 7), "Alice");
    }

    #[test]
    fn example2() {
        assert_eq!(losing_player(4, 11), "Bob");
    }
}
