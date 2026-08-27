/// LeetCode #3446 - Sort Matrix by Diagonals
fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    use std::collections::HashMap;
    let mut diags: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            diags.entry(i as i32 - j as i32).or_default().push(grid[i][j]);
        }
    }
    for (d, vals) in diags.iter_mut() {
        if *d >= 0 {
            vals.sort_unstable_by(|a, b| b.cmp(a));
        } else {
            vals.sort_unstable();
        }
    }
    let mut idx: HashMap<i32, usize> = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            let d = i as i32 - j as i32;
            let k = idx.entry(d).or_insert(0);
            grid[i][j] = diags[&d][*k];
            *k += 1;
        }
    }
    grid
}

fn main() {
    println!(
        "{:?}",
        sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]])
    );
}

#[cfg(test)]
mod tests {
    use super::sort_matrix;

    #[test]
    fn example1() {
        assert_eq!(
            sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]]),
            vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            sort_matrix(vec![vec![0, 1], vec![1, 2]]),
            vec![vec![2, 1], vec![1, 0]]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(sort_matrix(vec![vec![1]]), vec![vec![1]]);
    }
}
