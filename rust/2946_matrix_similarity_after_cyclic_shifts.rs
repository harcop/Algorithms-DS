/// LeetCode #2946 - Matrix Similarity After Cyclic Shifts
fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let n = mat[0].len();
    let k = k as usize % n;
    for (i, row) in mat.iter().enumerate() {
        for (j, &x) in row.iter().enumerate() {
            if i % 2 == 1 && x != mat[i][(j + k) % n] {
                return false;
            }
            if i % 2 == 0 && x != mat[i][(j + n - k) % n] {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        are_similar(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::are_similar;

    #[test]
    fn example_one() {
        assert!(!are_similar(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            4
        ));
    }

    #[test]
    fn example_two() {
        assert!(are_similar(
            vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]],
            2
        ));
    }

    #[test]
    fn example_three() {
        assert!(are_similar(vec![vec![2, 2], vec![2, 2]], 3));
    }
}
