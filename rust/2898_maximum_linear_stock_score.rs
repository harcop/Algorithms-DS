/// LeetCode #2898 - Maximum Linear Stock Score
fn max_score(prices: Vec<i32>) -> i64 {
    use std::collections::HashMap;

    let mut sums = HashMap::new();
    for (i, &price) in prices.iter().enumerate() {
        *sums.entry(price as i64 - i as i64).or_insert(0i64) += price as i64;
    }
    *sums.values().max().unwrap_or(&0)
}

fn main() {
    println!("{}", max_score(vec![1, 5, 3, 7, 8]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(max_score(vec![1, 5, 3, 7, 8]), 20);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_score(vec![5, 6, 7, 8, 9]), 35);
    }
}
