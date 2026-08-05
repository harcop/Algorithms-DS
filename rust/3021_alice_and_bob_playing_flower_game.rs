/// LeetCode #3021 - Alice and Bob Playing Flower Game
fn flower_game(n: i32, m: i32) -> i64 {
    let n = n as i64;
    let m = m as i64;
    (n + 1) / 2 * (m / 2) + (n / 2) * ((m + 1) / 2)
}

fn main() {
    println!("{}", flower_game(3, 2));
    println!("{}", flower_game(1, 1));
}

#[cfg(test)]
mod tests {
    use super::flower_game;

    #[test]
    fn example_one() {
        assert_eq!(flower_game(3, 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(flower_game(1, 1), 0);
    }
}
