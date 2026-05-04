/// LeetCode #292 - Nim Game
fn can_win_nim(n: i32) -> bool {
    n % 4 != 0
}

fn main() {
    println!("{}", can_win_nim(4));
}

#[cfg(test)]
mod tests {
    use super::can_win_nim;

    #[test]
    fn example_one() {
        assert!(!can_win_nim(4));
    }

    #[test]
    fn example_two() {
        assert!(can_win_nim(1));
    }
}
