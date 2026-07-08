/// LeetCode #2294 - Partition Array Such That Maximum Difference Is K
fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut ans = 1;
    let mut start = nums[0];
    for &x in nums.iter().skip(1) {
        if x - start > k {
            ans += 1;
            start = x;
        }
    }
    ans
}

fn main() {
    println!("{}", partition_array(vec![3, 6, 1, 2, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::partition_array;

    #[test]
    fn example_one() {
        assert_eq!(partition_array(vec![3, 6, 1, 2, 5], 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(partition_array(vec![1, 2, 3], 1), 2);
    }
}
