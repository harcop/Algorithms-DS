/// LeetCode #3107 - Minimum Operations to Make Median of Array Equal to K
fn min_operations_to_make_median_k(mut nums: Vec<i32>, k: i32) -> i64 {
    nums.sort_unstable();
    let n = nums.len();
    let m = n >> 1;
    let mut ans = (nums[m] - k).abs() as i64;
    if nums[m] > k {
        let mut i = m;
        while i > 0 {
            i -= 1;
            if nums[i] <= k {
                break;
            }
            ans += (nums[i] - k) as i64;
        }
    } else {
        for i in (m + 1)..n {
            if nums[i] >= k {
                break;
            }
            ans += (k - nums[i]) as i64;
        }
    }
    ans
}

fn main() {
    println!("{}", min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 4));
}

#[cfg(test)]
mod tests {
    use super::min_operations_to_make_median_k;

    #[test]
    fn example1() {
        assert_eq!(min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 4), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 7), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations_to_make_median_k(vec![1, 2, 3, 4, 5, 6], 4), 0);
    }
}
