/// LeetCode #2501 - Longest Square Streak in an Array
use std::collections::HashSet;

fn longest_square_streak(nums: Vec<i32>) -> i32 {
    let set: HashSet<i64> = nums.iter().map(|&x| x as i64).collect();
    let mut ans = -1;
    for &num in &set {
        let mut x = num;
        let mut t = 0;
        while set.contains(&x) {
            t += 1;
            if x > 100_000 {
                break;
            }
            x *= x;
        }
        if t > 1 {
            ans = ans.max(t);
        }
    }
    ans
}

fn main() {
    println!("{}", longest_square_streak(vec![4, 3, 6, 16, 8, 2]));
}

#[cfg(test)]
mod tests {
    use super::longest_square_streak;

    #[test]
    fn example_one() {
        assert_eq!(longest_square_streak(vec![4, 3, 6, 16, 8, 2]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_square_streak(vec![2, 3, 5, 6, 7]), -1);
    }
}
