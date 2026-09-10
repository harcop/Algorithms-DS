/// LeetCode #3678 - Smallest Absent Positive Greater Than Average
use std::collections::HashSet;

fn smallest_absent(nums: Vec<i32>) -> i32 {
    let s: HashSet<i32> = nums.iter().copied().collect();
    let sum: i32 = nums.iter().sum();
    let mut ans = 1.max(sum / nums.len() as i32 + 1);
    while s.contains(&ans) {
        ans += 1;
    }
    ans
}

fn main() {
    println!("{}", smallest_absent(vec![3, 5]));
}

#[cfg(test)]
mod tests {
    use super::smallest_absent;

    #[test]
    fn example1() {
        assert_eq!(smallest_absent(vec![3, 5]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_absent(vec![-1, 1, 2]), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(smallest_absent(vec![4, -1]), 2);
    }
}
