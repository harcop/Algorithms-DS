/// LeetCode #3724 - Minimum Operations to Transform Array
fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let n = nums1.len();
    let last = nums2[n];
    let mut ans = 1i64;
    let mut ok = false;
    let mut d = i32::MAX;
    for i in 0..n {
        let x = nums1[i].max(nums2[i]);
        let y = nums1[i].min(nums2[i]);
        ans += (x - y) as i64;
        d = d.min((x - last).abs()).min((y - last).abs());
        if last >= y && last <= x {
            ok = true;
        }
    }
    if !ok {
        ans += d as i64;
    }
    ans
}

fn main() {
    println!("{}", min_operations(vec![2, 8], vec![1, 7, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![2, 8], vec![1, 7, 3]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 3, 6], vec![2, 4, 5, 3]), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![2], vec![3, 4]), 3);
    }
}
