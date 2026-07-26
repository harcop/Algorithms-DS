/// LeetCode #2713 - Maximum Strictly Increasing Cells in a Matrix
use std::collections::BTreeMap;

fn max_increasing_cells(mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut g: BTreeMap<i32, Vec<(usize, usize)>> = BTreeMap::new();
    for i in 0..m {
        for j in 0..n {
            g.entry(mat[i][j]).or_default().push((i, j));
        }
    }
    let mut row_max = vec![0; m];
    let mut col_max = vec![0; n];
    let mut ans = 0;
    for (_val, pos) in g {
        let mx: Vec<i32> = pos
            .iter()
            .map(|&(i, j)| 1 + row_max[i].max(col_max[j]))
            .collect();
        for &v in &mx {
            ans = ans.max(v);
        }
        for (k, &(i, j)) in pos.iter().enumerate() {
            row_max[i] = row_max[i].max(mx[k]);
            col_max[j] = col_max[j].max(mx[k]);
        }
    }
    ans
}

fn main() {
    println!("{}", max_increasing_cells(vec![vec![3, 1], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::max_increasing_cells;

    #[test]
    fn example_one() {
        assert_eq!(max_increasing_cells(vec![vec![3, 1], vec![3, 4]]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_increasing_cells(vec![vec![1, 1], vec![1, 1]]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            max_increasing_cells(vec![vec![3, 1, 6], vec![-9, 5, 7]]),
            4
        );
    }
}
