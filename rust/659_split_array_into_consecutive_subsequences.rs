/// LeetCode #659 - Split Array into Consecutive Subsequences
use std::collections::HashMap;

fn is_possible(nums: Vec<i32>) -> bool {
    let mut freq = HashMap::new();
    for &x in &nums {
        *freq.entry(x).or_insert(0) += 1;
    }
    let mut tails = HashMap::new();
    for x in nums {
        if *freq.get(&x).unwrap_or(&0) == 0 {
            continue;
        }
        *freq.get_mut(&x).unwrap() -= 1;
        if *tails.get(&(x - 1)).unwrap_or(&0) > 0 {
            *tails.get_mut(&(x - 1)).unwrap() -= 1;
            *tails.entry(x).or_insert(0) += 1;
        } else if *freq.get(&(x + 1)).unwrap_or(&0) > 0 && *freq.get(&(x + 2)).unwrap_or(&0) > 0 {
            *freq.get_mut(&(x + 1)).unwrap() -= 1;
            *freq.get_mut(&(x + 2)).unwrap() -= 1;
            *tails.entry(x + 2).or_insert(0) += 1;
        } else {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_possible(vec![1, 2, 3, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::is_possible;

    #[test]
    fn example_one() {
        assert!(is_possible(vec![1, 2, 3, 3, 4, 5]));
    }

    #[test]
    fn example_two() {
        assert!(is_possible(vec![1, 2, 3, 3, 4, 4, 5, 6]));
    }

    #[test]
    fn example_three() {
        assert!(!is_possible(vec![1, 2, 3, 4, 4, 5]));
    }
}
