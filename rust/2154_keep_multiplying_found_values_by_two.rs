/// LeetCode #2154 - Keep Multiplying Found Values by Two
use std::collections::HashSet;

fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let seen: HashSet<i32> = nums.into_iter().collect();
    let mut ans = original;
    while seen.contains(&ans) {
        ans *= 2;
    }
    ans
}

fn main() {
    println!("{}", find_final_value(vec![5, 3, 6, 1, 12], 3));
}

#[cfg(test)]
mod tests {
    use super::find_final_value;

    #[test]
    fn example_one() {
        assert_eq!(find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_final_value(vec![2, 7, 9], 4), 4);
    }
}
