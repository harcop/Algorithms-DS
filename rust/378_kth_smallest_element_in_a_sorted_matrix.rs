/// LeetCode #378 - kth smallest in sorted matrix (binary search on value)
fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = matrix.len();
    let k = k as usize;
    let mut lo = *matrix.iter().flatten().min().unwrap();
    let mut hi = *matrix.iter().flatten().max().unwrap();
    fn count(mat: &[Vec<i32>], x: i32, n: usize) -> usize {
        let mut c = 0usize;
        let mut r = (n - 1) as isize;
        let mut ccol = 0usize;
        while r >= 0 && ccol < n {
            if mat[r as usize][ccol] <= x {
                c += (r + 1) as usize;
                ccol += 1;
            } else {
                r -= 1;
            }
        }
        c
    }
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(&matrix, mid, n) < k {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    println!(
        "{}",
        kth_smallest(
            vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]],
            8
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(
            kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
    }
}
