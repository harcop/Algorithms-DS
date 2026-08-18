/// LeetCode #3269 - Constructing Two Increasing Arrays
fn min_largest(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let nxt = |x: i32, y: i32| -> i32 {
        if (x & 1) ^ y == 1 {
            x + 1
        } else {
            x + 2
        }
    };
    let m = nums1.len();
    let n = nums2.len();
    let mut f = vec![vec![0; n + 1]; m + 1];
    for (i, &x) in nums1.iter().enumerate() {
        f[i + 1][0] = nxt(f[i][0], x);
    }
    for (j, &y) in nums2.iter().enumerate() {
        f[0][j + 1] = nxt(f[0][j], y);
    }
    for (i, &x) in nums1.iter().enumerate() {
        for (j, &y) in nums2.iter().enumerate() {
            f[i + 1][j + 1] = nxt(f[i][j + 1], x).min(nxt(f[i + 1][j], y));
        }
    }
    f[m][n]
}

fn main() {
    println!("{}", min_largest(vec![], vec![1, 0, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_largest;

    #[test]
    fn example1() {
        assert_eq!(min_largest(vec![], vec![1, 0, 1, 1]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(min_largest(vec![0, 1, 0, 1], vec![1, 0, 0, 1]), 9);
    }

    #[test]
    fn example3() {
        assert_eq!(min_largest(vec![0, 1, 0, 0, 1], vec![0, 0, 0, 1]), 13);
    }
}
