/// LeetCode #1329 - Sort the Matrix Diagonally
fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = mat.len();
    let n = mat[0].len();
    let mut out = mat.clone();
    for k in -(m as i32 - 1)..n as i32 {
        let mut cells = vec![];
        for r in 0..m {
            let c = r as i32 + k;
            if c >= 0 && c < n as i32 {
                cells.push((r, c as usize));
            }
        }
        let mut vals: Vec<i32> = cells.iter().map(|&(r, c)| mat[r][c]).collect();
        vals.sort_unstable();
        for (i, &(r, c)) in cells.iter().enumerate() {
            out[r][c] = vals[i];
        }
    }
    out
}

fn main() {
    println!("{:?}", diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::diagonal_sort;

    #[test]
    fn example_one() {
        assert_eq!(
            diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
        );
    }
}
