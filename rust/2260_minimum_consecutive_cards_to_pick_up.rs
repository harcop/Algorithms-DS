/// LeetCode #2260 - Minimum Consecutive Cards to Pick Up
use std::collections::HashMap;

fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut last_seen: HashMap<i32, usize> = HashMap::new();
    let mut ans = usize::MAX;

    for (i, &card) in cards.iter().enumerate() {
        if let Some(&prev) = last_seen.get(&card) {
            ans = ans.min(i - prev + 1);
        }
        last_seen.insert(card, i);
    }

    if ans == usize::MAX {
        -1
    } else {
        ans as i32
    }
}

fn main() {
    println!("{}", minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]));
}

#[cfg(test)]
mod tests {
    use super::minimum_card_pickup;

    #[test]
    fn example_one() {
        assert_eq!(minimum_card_pickup(vec![3, 4, 2, 3, 4, 7]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_card_pickup(vec![1, 0, 5, 3]), -1);
    }
}
