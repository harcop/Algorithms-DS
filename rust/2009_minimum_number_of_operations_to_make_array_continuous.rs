/// LeetCode #2009 - Minimum Number of Operations to Make Array Continuous
fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut uniq = nums;
    uniq.sort_unstable();
    uniq.dedup();
    let mut ans = n;
    for (i, &v) in uniq.iter().enumerate() {
        let j = uniq.partition_point(|&x| x <= v + n as i32 - 1);
        ans = ans.min(n - (j - i));
    }
    ans as i32
}

fn main() {
    println!("{}", min_operations(vec![4, 2, 5, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![4, 2, 5, 3]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![1, 2, 3, 5, 6]), 1);
    }
}
