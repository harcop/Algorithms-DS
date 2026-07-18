/// LeetCode #2489 - Number of Substrings With Fixed Ratio
use std::collections::HashMap;

fn fixed_ratio(s: String, num1: i32, num2: i32) -> i64 {
    let mut answer = 0i64;
    let mut prefix = 0i64;
    let mut prefix_count = HashMap::from([(0i64, 1i64)]);

    for c in s.chars() {
        if c == '0' {
            prefix += num2 as i64;
        } else {
            prefix -= num1 as i64;
        }
        answer += prefix_count.get(&prefix).copied().unwrap_or(0);
        *prefix_count.entry(prefix).or_insert(0) += 1;
    }

    answer
}

fn main() {
    println!("{}", fixed_ratio("0110011".to_string(), 1, 2));
}

#[cfg(test)]
mod tests {
    use super::fixed_ratio;

    #[test]
    fn example_one() {
        assert_eq!(fixed_ratio("0110011".to_string(), 1, 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(fixed_ratio("10101".to_string(), 3, 1), 0);
    }
}
