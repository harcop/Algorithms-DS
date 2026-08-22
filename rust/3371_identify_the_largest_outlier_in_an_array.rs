/// LeetCode #3371 - Identify the Largest Outlier in an Array
use std::collections::HashMap;

fn get_largest_outlier(nums: Vec<i32>) -> i32 {
    let s: i32 = nums.iter().sum();
    let mut cnt = HashMap::new();
    for &x in &nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut ans = i32::MIN;
    for (&x, &v) in &cnt {
        let t = s - x;
        if t % 2 != 0 {
            continue;
        }
        let half = t / 2;
        let c = *cnt.get(&half).unwrap_or(&0);
        if c == 0 {
            continue;
        }
        if x != half || v > 1 {
            ans = ans.max(x);
        }
    }
    ans
}

fn main() {
    println!("{}", get_largest_outlier(vec![2, 3, 5, 10]));
}

#[cfg(test)]
mod tests {
    use super::get_largest_outlier;

    #[test]
    fn example1() {
        assert_eq!(get_largest_outlier(vec![2, 3, 5, 10]), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(get_largest_outlier(vec![-2, -1, -3, -6, 4]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(get_largest_outlier(vec![1, 1, 1, 1, 1, 5, 5]), 5);
    }
}
