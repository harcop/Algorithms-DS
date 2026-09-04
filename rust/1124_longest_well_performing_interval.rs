/// LeetCode #1124 - Longest Well-Performing Interval
use std::collections::HashMap;

fn longest_wpi(hours: Vec<i32>) -> i32 {
    let mut prefix = 0i32;
    let mut first = HashMap::new();
    let mut ans = 0i32;
    for (i, &h) in hours.iter().enumerate() {
        prefix += if h > 8 { 1 } else { -1 };
        if prefix > 0 {
            ans = i as i32 + 1;
        } else {
            first.entry(prefix).or_insert(i);
            if let Some(&j) = first.get(&(prefix - 1)) {
                ans = ans.max(i as i32 - j as i32);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]));
}

#[cfg(test)]
mod tests {
    use super::longest_wpi;

    #[test]
    fn example_one() {
        assert_eq!(longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_wpi(vec![6, 6, 6]), 0);
    }
}
