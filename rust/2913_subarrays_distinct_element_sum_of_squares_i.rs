/// LeetCode #2913 - Subarrays Distinct Element Sum of Squares I
fn sum_counts(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        let mut seen = HashSet::new();
        for j in i..n {
            seen.insert(nums[j]);
            let d = seen.len() as i32;
            ans += d * d;
        }
    }
    ans
}

fn main() {
    println!("{}", sum_counts(vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::sum_counts;

    #[test]
    fn example_one() {
        assert_eq!(sum_counts(vec![1, 2, 1]), 15);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_counts(vec![1, 1]), 3);
    }
}
