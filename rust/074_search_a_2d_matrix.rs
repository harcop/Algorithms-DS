/// LeetCode #74 - Search a 2D Matrix
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }
    let n = matrix[0].len();
    let mut lo = 0usize;
    let mut hi = matrix.len() * n;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let val = matrix[mid / n][mid % n];
        if val < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo < matrix.len() * n && matrix[lo / n][lo % n] == target
}

fn main() {
    println!(
        "{}",
        search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        )
    );
}

#[cfg(test)]
mod tests {
    use super::search_matrix;

    #[test]
    fn example_one() {
        assert!(search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
    }

    #[test]
    fn example_two() {
        assert!(!search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }
}
