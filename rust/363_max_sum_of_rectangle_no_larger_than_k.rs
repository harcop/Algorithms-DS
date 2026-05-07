/// LeetCode #363 - Max Sum of Rectangle No Larger Than K (2D prefix columns + BST on running sum)
use std::collections::BTreeSet;

fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut best = i32::MIN;
    let mut col_sum = vec![0i32; rows];
    for left in 0..cols {
        col_sum.fill(0);
        for right in left..cols {
            for r in 0..rows {
                col_sum[r] += matrix[r][right];
            }
            let mut prefix = 0i32;
            let mut sums = BTreeSet::new();
            sums.insert(0);
            for &v in &col_sum {
                prefix += v;
                if let Some(&low) = sums.range(..=prefix - k).next_back() {
                    best = best.max(prefix - low);
                    if best == k {
                        return k;
                    }
                }
                sums.insert(prefix);
            }
        }
    }
    best
}

fn main() {
    let m = vec![vec![1, 0, 1], vec![0, -2, 3]];
    println!("{}", max_sum_submatrix(m, 2));
}

#[cfg(test)]
mod tests {
    use super::max_sum_submatrix;

    #[test]
    fn ex() {
        assert_eq!(
            max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
            2
        );
    }
}
