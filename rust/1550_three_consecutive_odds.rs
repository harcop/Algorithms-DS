/// LeetCode #1550 - Three Consecutive Odds
fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    arr.windows(3).any(|w| w.iter().all(|&x| x % 2 == 1))
}

fn main() {
    println!("{}", three_consecutive_odds(vec![2, 6, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::three_consecutive_odds;

    #[test]
    fn example_one() {
        assert!(!three_consecutive_odds(vec![2, 6, 4, 1]));
    }

    #[test]
    fn example_two() {
        assert!(three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]));
    }
}
