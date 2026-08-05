/// LeetCode #3005 - Count Elements With Maximum Frequency
use std::collections::HashMap;

fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut freq = HashMap::new();
    for &x in &nums {
        *freq.entry(x).or_insert(0) += 1;
    }
    let max_freq = freq.values().copied().max().unwrap_or(0);
    freq.values().filter(|&&c| c == max_freq).sum()
}

fn main() {
    println!("{}", max_frequency_elements(vec![1, 2, 2, 3, 1, 4]));
    println!("{}", max_frequency_elements(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_frequency_elements;

    #[test]
    fn example_one() {
        assert_eq!(max_frequency_elements(vec![1, 2, 2, 3, 1, 4]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_frequency_elements(vec![1, 2, 3, 4, 5]), 5);
    }
}
