/// LeetCode #575 - Distribute Candies
use std::collections::HashSet;

fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let u = candy_type.iter().copied().collect::<HashSet<_>>().len() as i32;
    let half = candy_type.len() as i32 / 2;
    u.min(half)
}

fn main() {
    println!("{}", distribute_candies(vec![1, 1, 2, 2, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::distribute_candies;

    #[test]
    fn example_one() {
        assert_eq!(distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(distribute_candies(vec![1, 1, 2, 3]), 2);
    }
}
