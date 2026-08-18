/// LeetCode #3257 - Maximum Value Sum by Placing Three Rooks II
fn maximum_value_sum(board: Vec<Vec<i32>>) -> i64 {
    let m = board.len();
    let n = board[0].len();
    let mut row_best = Vec::new();
    for (i, row) in board.iter().enumerate() {
        let mut cells: Vec<(i64, usize, usize)> = row
            .iter()
            .enumerate()
            .map(|(j, &v)| (v as i64, i, j))
            .collect();
        cells.sort_by(|a, b| b.0.cmp(&a.0));
        cells.truncate(3);
        row_best.extend(cells);
    }
    let mut col_best = Vec::new();
    for j in 0..n {
        let mut cells: Vec<(i64, usize, usize)> = (0..m)
            .map(|i| (board[i][j] as i64, i, j))
            .collect();
        cells.sort_by(|a, b| b.0.cmp(&a.0));
        cells.truncate(3);
        col_best.extend(cells);
    }
    use std::collections::HashSet;
    let rs: HashSet<_> = row_best.into_iter().collect();
    let cs: HashSet<_> = col_best.into_iter().collect();
    let mut cand: Vec<_> = rs.intersection(&cs).cloned().collect();
    cand.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    cand.truncate(11);
    let mut ans = i64::MIN;
    let t = cand.len();
    for a in 0..t {
        for b in a + 1..t {
            for c in b + 1..t {
                let (v1, i1, j1) = cand[a];
                let (v2, i2, j2) = cand[b];
                let (v3, i3, j3) = cand[c];
                if i1 != i2 && i1 != i3 && i2 != i3 && j1 != j2 && j1 != j3 && j2 != j3 {
                    ans = ans.max(v1 + v2 + v3);
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_value_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_value_sum;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_value_sum(vec![
                vec![-3, 1, 1, 1],
                vec![-3, 1, -3, 1],
                vec![-3, 2, 1, 1]
            ]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_value_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            15
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            maximum_value_sum(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]),
            3
        );
    }
}
