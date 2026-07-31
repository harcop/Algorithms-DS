/// LeetCode #2817 - Minimum Absolute Difference Between Elements With Constraint
use std::collections::BTreeSet;

fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
    let mut window = BTreeSet::new();
    let mut ans = i32::MAX;
    for i in 0..nums.len() {
        if i >= x as usize {
            window.insert((nums[i - x as usize], i - x as usize));
        }
        if i >= x as usize {
            let target = nums[i];
            if let Some(&(v, _)) = window.range((i32::MIN, 0)..=(target, usize::MAX)).next_back() {
                ans = ans.min((target - v).abs());
            }
            if let Some(&(v, _)) = window.range((target, 0)..=(i32::MAX, usize::MAX)).next() {
                ans = ans.min((target - v).abs());
            }
        }
    }
    ans
}

fn main() {
    println!("{}", min_absolute_difference(vec![4, 3, 2, 4], 2));
}

#[cfg(test)]
mod tests {
    use super::min_absolute_difference;

    #[test]
    fn example_one() {
        assert_eq!(min_absolute_difference(vec![4, 3, 2, 4], 2), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_absolute_difference(vec![5, 3, 2, 10, 15], 1), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_absolute_difference(vec![1, 2, 3, 4], 3), 3);
    }
}
