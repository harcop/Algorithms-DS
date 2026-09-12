/// LeetCode #3708 - Longest Fibonacci Subarray
fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut result = 2;
    let mut cnt = 2;
    for i in 2..nums.len() {
        if nums[i] != nums[i - 1] + nums[i - 2] {
            cnt = 2;
            continue;
        }
        cnt += 1;
        result = result.max(cnt);
    }
    result
}

fn main() {
    println!("{}", longest_subarray(vec![1, 1, 1, 1, 2, 3, 5, 1]));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example1() {
        assert_eq!(longest_subarray(vec![1, 1, 1, 1, 2, 3, 5, 1]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subarray(vec![5, 2, 7, 9, 16]), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(
            longest_subarray(vec![1_000_000_000, 1_000_000_000, 1_000_000_000]),
            2
        );
    }
}
