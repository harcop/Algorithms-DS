/// LeetCode #240 - Search a 2D Matrix II
fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() {
        return false;
    }
    let m = matrix.len();
    let n = matrix[0].len();
    let mut i = 0i32;
    let mut j = n as i32 - 1;
    while i < m as i32 && j >= 0 {
        let v = matrix[i as usize][j as usize];
        if v == target {
            return true;
        }
        if v > target {
            j -= 1;
        } else {
            i += 1;
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        search_matrix(
            vec![
                vec![1, 4, 7, 11, 15],
                vec![2, 5, 8, 12, 19],
                vec![3, 6, 9, 16, 22],
                vec![10, 13, 14, 17, 24],
                vec![18, 21, 23, 26, 30],
            ],
            5
        )
    );
}

#[cfg(test)]
mod tests {
    use super::search_matrix;

    #[test]
    fn example_one() {
        let m = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(search_matrix(m, 5));
    }

    #[test]
    fn example_two() {
        let m = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(!search_matrix(m, 20));
    }
}
