/// LeetCode #2144 - Minimum Cost of Buying Candies With Discount
fn minimum_cost(cost: Vec<i32>) -> i32 {
    let mut cost = cost;
    cost.sort_unstable_by(|a, b| b.cmp(a));

    cost.iter()
        .enumerate()
        .filter(|(i, _)| i % 3 != 2)
        .map(|(_, &value)| value)
        .sum()
}

fn main() {
    println!("{}", minimum_cost(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example_one() {
        assert_eq!(minimum_cost(vec![1, 2, 3]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_cost(vec![5, 5]), 10);
    }
}
