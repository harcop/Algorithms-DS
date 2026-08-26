/// LeetCode #3432 - Count Partitions with Even Sum Difference
fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right: i32 = nums.iter().sum();
    let mut ans = 0;
    for &x in &nums[..nums.len() - 1] {
        left += x;
        right -= x;
        if (left - right) % 2 == 0 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_partitions(vec![10, 10, 3, 7, 6]));
}

#[cfg(test)]
mod tests {
    use super::count_partitions;

    #[test]
    fn example1() {
        assert_eq!(count_partitions(vec![10, 10, 3, 7, 6]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_partitions(vec![1, 2, 2]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(count_partitions(vec![2, 4, 6, 8]), 3);
    }
}
