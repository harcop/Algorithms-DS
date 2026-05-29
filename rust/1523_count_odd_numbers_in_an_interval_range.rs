/// LeetCode #1523 - Count Odd Numbers In An Interval Range
fn count_odds(low: i32, high: i32) -> i32 {
    fn odds(x: i32) -> i32 {
        (x + 1) / 2
    }
    odds(high) - odds(low - 1)
}

fn main() {
    println!("{}", count_odds(3, 7));
}

#[cfg(test)]
mod tests {
    use super::count_odds;

    #[test]
    fn example_one() {
        assert_eq!(count_odds(3, 7), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_odds(8, 10), 1);
    }
}
