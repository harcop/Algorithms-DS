/// LeetCode #2670 - Find the Distinct Difference Array
use std::collections::HashSet;

fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut suf = vec![0; n + 1];
    let mut s = HashSet::new();
    for i in (0..n).rev() {
        s.insert(nums[i]);
        suf[i] = s.len();
    }
    s.clear();
    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        s.insert(nums[i]);
        ans.push(s.len() as i32 - suf[i + 1] as i32);
    }
    ans
}

fn main() {
    println!("{:?}", distinct_difference_array(vec![1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::distinct_difference_array;

    #[test]
    fn example_one() {
        assert_eq!(
            distinct_difference_array(vec![1, 2, 3, 4, 5]),
            vec![-3, -1, 1, 3, 5]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            distinct_difference_array(vec![3, 2, 3, 4, 2]),
            vec![-2, -1, 0, 2, 3]
        );
    }
}
