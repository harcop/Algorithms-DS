/// LeetCode #2962 - Count Subarrays Where Max Element Appears at Least K Times
fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let mx = *nums.iter().max().unwrap();
    let n = nums.len();
    let mut ans = 0i64;
    let mut cnt = 0;
    let mut j = 0usize;
    for &x in &nums {
        while j < n && cnt < k {
            if nums[j] == mx {
                cnt += 1;
            }
            j += 1;
        }
        if cnt < k {
            break;
        }
        ans += (n - j + 1) as i64;
        if x == mx {
            cnt -= 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_subarrays(vec![1, 3, 2, 3, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example_one() {
        assert_eq!(count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_subarrays(vec![1, 4, 2, 1], 3), 0);
    }
}
