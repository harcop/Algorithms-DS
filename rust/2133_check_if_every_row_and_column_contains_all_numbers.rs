/// LeetCode #2133 - Check if Every Row and Column Contains All Numbers
fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    let n = matrix.len();

    for i in 0..n {
        let mut row = vec![false; n + 1];
        let mut col = vec![false; n + 1];

        for j in 0..n {
            let r = matrix[i][j] as usize;
            let c = matrix[j][i] as usize;

            if r == 0 || r > n || c == 0 || c > n || row[r] || col[c] {
                return false;
            }

            row[r] = true;
            col[c] = true;
        }
    }

    true
}

fn main() {
    println!(
        "{}",
        check_valid(vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::check_valid;

    #[test]
    fn example_one() {
        assert!(check_valid(vec![
            vec![1, 2, 3],
            vec![3, 1, 2],
            vec![2, 3, 1]
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!check_valid(vec![
            vec![1, 1, 1],
            vec![1, 2, 3],
            vec![1, 2, 3]
        ]));
    }
}
