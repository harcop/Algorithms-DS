/// LeetCode #950 - Reveal Cards In Increasing Order
use std::collections::VecDeque;

fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
    let mut deck = deck;
    deck.sort_unstable();
    let mut q = VecDeque::new();
    for i in 0..deck.len() {
        q.push_back(i);
    }
    let mut ans = vec![0i32; deck.len()];
    for &card in &deck {
        let idx = q.pop_front().unwrap();
        if let Some(&next) = q.front() {
            q.push_back(next);
            q.pop_front();
        }
        ans[idx] = card;
    }
    ans
}

fn main() {
    println!("{:?}", deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]));
}

#[cfg(test)]
mod tests {
    use super::deck_revealed_increasing;

    #[test]
    fn example_one() {
        assert_eq!(
            deck_revealed_increasing(vec![17, 13, 11, 2, 3, 5, 7]),
            vec![2, 13, 3, 11, 5, 17, 7]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(deck_revealed_increasing(vec![1, 1000]), vec![1, 1000]);
    }
}
