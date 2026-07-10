/// LeetCode #2347 - Best Poker Hand
fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
    if suits.iter().all(|&v| v == suits[0]) {
        return "Flush".to_string();
    }
    let mut count = [0; 14];
    let mut is_pair = false;
    for &v in &ranks {
        let i = v as usize;
        count[i] += 1;
        if count[i] == 3 {
            return "Three of a Kind".to_string();
        }
        is_pair = is_pair || count[i] == 2;
    }
    if is_pair {
        "Pair".to_string()
    } else {
        "High Card".to_string()
    }
}

fn main() {
    println!(
        "{}",
        best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a'])
    );
}

#[cfg(test)]
mod tests {
    use super::best_hand;

    #[test]
    fn example_one() {
        assert_eq!(
            best_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            best_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
            "Three of a Kind"
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            best_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "Pair"
        );
    }
}
