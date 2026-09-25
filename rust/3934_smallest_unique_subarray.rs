/// LeetCode #3934 - Smallest Unique Subarray
use std::collections::HashMap;

fn has_unique(nums: &[i32], len: usize) -> bool {
    const M1: i64 = 1_000_000_007;
    const M2: i64 = 1_000_000_009;
    const B1: i64 = 911_382_323;
    const B2: i64 = 972_663_749;
    let n = nums.len();
    let mut p1 = vec![1i64; n + 1];
    let mut p2 = vec![1i64; n + 1];
    for i in 1..=n {
        p1[i] = p1[i - 1] * B1 % M1;
        p2[i] = p2[i - 1] * B2 % M2;
    }
    let mut h1 = 0i64;
    let mut h2 = 0i64;
    for i in 0..len {
        h1 = (h1 * B1 + nums[i] as i64) % M1;
        h2 = (h2 * B2 + nums[i] as i64) % M2;
    }
    let mut cnt: HashMap<(i64, i64), i32> = HashMap::new();
    *cnt.entry((h1, h2)).or_insert(0) += 1;
    for i in 1..=n - len {
        h1 = (h1 - nums[i - 1] as i64 * p1[len - 1]).rem_euclid(M1);
        h1 = (h1 * B1 + nums[i + len - 1] as i64) % M1;
        h2 = (h2 - nums[i - 1] as i64 * p2[len - 1]).rem_euclid(M2);
        h2 = (h2 * B2 + nums[i + len - 1] as i64) % M2;
        *cnt.entry((h1, h2)).or_insert(0) += 1;
    }
    cnt.values().any(|&c| c == 1)
}

fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut lo = 1usize;
    let mut hi = n;
    let mut ans = n;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if has_unique(&nums, mid) {
            ans = mid;
            if mid == 1 {
                break;
            }
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", smallest_unique_subarray(vec![3, 3, 3]));
}

#[cfg(test)]
mod tests {
    use super::smallest_unique_subarray;

    #[test]
    fn example1() {
        assert_eq!(smallest_unique_subarray(vec![3, 3, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(smallest_unique_subarray(vec![2, 1, 2, 3, 3]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(smallest_unique_subarray(vec![1, 1, 2, 2, 1]), 2);
    }
}
