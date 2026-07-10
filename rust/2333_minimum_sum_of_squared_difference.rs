/// LeetCode #2333 - Minimum Sum of Squared Difference
fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let n = nums1.len();
    let mut d = vec![0i32; n];
    let mut s = 0i64;
    let mut mx = 0i32;
    let mut k = k1 + k2;

    for i in 0..n {
        d[i] = (nums1[i] - nums2[i]).abs();
        s += d[i] as i64;
        mx = mx.max(d[i]);
    }
    if s <= k as i64 {
        return 0;
    }

    let mut left = 0;
    let mut right = mx;
    while left < right {
        let mid = (left + right) >> 1;
        let mut t = 0i64;
        for &v in &d {
            t += (v - mid).max(0) as i64;
        }
        if t <= k as i64 {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    for i in 0..n {
        k -= (d[i] - left).max(0);
        d[i] = d[i].min(left);
    }
    for i in 0..n {
        if k <= 0 {
            break;
        }
        if d[i] == left {
            d[i] -= 1;
            k -= 1;
        }
    }

    d.iter().map(|&v| v as i64 * v as i64).sum()
}

fn main() {
    println!(
        "{}",
        min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0)
    );
}

#[cfg(test)]
mod tests {
    use super::min_sum_square_diff;

    #[test]
    fn example_one() {
        assert_eq!(
            min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
            579
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
            43
        );
    }
}
