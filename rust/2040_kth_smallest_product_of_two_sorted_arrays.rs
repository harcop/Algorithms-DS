/// LeetCode #2040 - Kth Smallest Product of Two Sorted Arrays
fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
    let count = |p: i64| -> i64 {
        let mut cnt = 0i64;
        let n = nums2.len();
        for &x in &nums1 {
            if x > 0 {
                cnt += nums2.partition_point(|&y| (y as i64) * (x as i64) <= p) as i64;
            } else if x < 0 {
                cnt += (n - nums2.partition_point(|&y| (y as i64) * (x as i64) > p)) as i64;
            } else {
                cnt += if p >= 0 { n as i64 } else { 0 };
            }
        }
        cnt
    };

    let mx = nums1.iter().map(|x| x.abs() as i64).max().unwrap()
        * nums2.iter().map(|x| x.abs() as i64).max().unwrap();

    let mut lo = -mx;
    let mut hi = mx;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(mid) >= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!(
        "{}",
        kth_smallest_product(vec![2, 5], vec![3, 4], 2)
    );
}

#[cfg(test)]
mod tests {
    use super::kth_smallest_product;

    #[test]
    fn example_one() {
        assert_eq!(kth_smallest_product(vec![2, 5], vec![3, 4], 2), 8);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            kth_smallest_product(vec![-4, -2, 0, 3], vec![2, 4], 6),
            0
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            kth_smallest_product(vec![-2, -1, 0, 1, 2], vec![-3, -1, 2, 4, 5], 3),
            -6
        );
    }
}
