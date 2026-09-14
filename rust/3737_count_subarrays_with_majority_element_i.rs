/// LeetCode #3737 - Count Subarrays With Majority Element I
fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        let mut cnt = 0;
        for j in i..n {
            if nums[j] == target {
                cnt += 1;
            }
            if cnt * 2 > (j - i + 1) as i32 {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", count_majority_subarrays(vec![1, 2, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::count_majority_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_majority_subarrays(vec![1, 2, 2, 3], 2), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(count_majority_subarrays(vec![1, 1, 1, 1], 1), 10);
    }

    #[test]
    fn example3() {
        assert_eq!(count_majority_subarrays(vec![1, 2, 3], 4), 0);
    }
}
