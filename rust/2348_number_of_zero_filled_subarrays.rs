/// LeetCode #2348 - Number of Zero-Filled Subarrays
fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut cnt = 0i64;
    for x in nums {
        if x == 0 {
            cnt += 1;
            ans += cnt;
        } else {
            cnt = 0;
        }
    }
    ans
}

fn main() {
    println!("{}", zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]));
}

#[cfg(test)]
mod tests {
    use super::zero_filled_subarray;

    #[test]
    fn example_one() {
        assert_eq!(zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]), 9);
    }

    #[test]
    fn example_three() {
        assert_eq!(zero_filled_subarray(vec![2, 10, 2019]), 0);
    }
}
