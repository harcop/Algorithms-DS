/// LeetCode #2740 - Find the Value of the Partition
fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut ans = i32::MAX;
    for i in 1..nums.len() {
        ans = ans.min(nums[i] - nums[i - 1]);
    }
    ans
}

fn main() {
    println!("{}", find_value_of_partition(vec![1, 3, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_value_of_partition;

    #[test]
    fn example_one() {
        assert_eq!(find_value_of_partition(vec![1, 3, 2, 4]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_value_of_partition(vec![100, 1, 10]), 9);
    }
}
