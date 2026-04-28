/// LeetCode #52 - N-Queens II
fn total_n_queens(n: i32) -> i32 {
    let n = n as usize;
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n - 1];
    let mut diag2 = vec![false; 2 * n - 1];

    fn backtrack(
        row: usize,
        n: usize,
        cols: &mut [bool],
        diag1: &mut [bool],
        diag2: &mut [bool],
    ) -> i32 {
        if row == n {
            return 1;
        }

        let mut count = 0;
        for col in 0..n {
            let d1 = row + col;
            let d2 = row + (n - 1 - col);
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;

            count += backtrack(row + 1, n, cols, diag1, diag2);

            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
        count
    }

    backtrack(0, n, &mut cols, &mut diag1, &mut diag2)
}

fn main() {
    println!("{}", total_n_queens(4));
}

#[cfg(test)]
mod tests {
    use super::total_n_queens;

    #[test]
    fn example_one() {
        assert_eq!(total_n_queens(4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(total_n_queens(1), 1);
    }
}
