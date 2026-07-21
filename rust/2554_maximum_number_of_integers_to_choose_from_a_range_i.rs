/// LeetCode #2554 - Maximum Number of Integers to Choose From a Range I
use std::collections::HashSet;

fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
    let banned: HashSet<i32> = banned.into_iter().collect();
    let mut ans = 0;
    let mut sum = 0i32;
    for i in 1..=n {
        if sum + i > max_sum {
            break;
        }
        if banned.contains(&i) {
            continue;
        }
        sum += i;
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", max_count(vec![1, 6, 5], 5, 6));
}

#[cfg(test)]
mod tests {
    use super::max_count;

    #[test]
    fn example_one() {
        assert_eq!(max_count(vec![1, 6, 5], 5, 6), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_count(vec![1, 2, 3, 4, 5, 6, 7], 8, 1), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(max_count(vec![11], 7, 50), 7);
    }
}
