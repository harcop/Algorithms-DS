/// LeetCode #1918 - Kth Smallest Subarray Sum
fn kth_smallest_subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let mut lo = *nums.iter().min().unwrap() as i64;
    let mut hi = nums.iter().map(|&x| x as i64).sum();

    let check = |s: i64| -> bool {
        let mut t = 0i64;
        let mut j = 0usize;
        let mut cnt = 0i64;
        for (i, &x) in nums.iter().enumerate() {
            t += x as i64;
            while t > s {
                t -= nums[j] as i64;
                j += 1;
            }
            cnt += (i + 1 - j) as i64;
        }
        cnt >= k
    };

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if check(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", kth_smallest_subarray_sum(vec![2, 1, 3], 4));
}

#[cfg(test)]
mod tests {
    use super::kth_smallest_subarray_sum;

    #[test]
    fn example_one() {
        assert_eq!(kth_smallest_subarray_sum(vec![2, 1, 3], 4), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(kth_smallest_subarray_sum(vec![3, 3, 4, 5], 7), 9);
    }
}
