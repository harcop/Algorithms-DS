/// LeetCode #2661 - First Completely Painted Row or Column
use std::collections::HashMap;

fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();
    let mut idx = HashMap::new();
    for i in 0..m {
        for j in 0..n {
            idx.insert(mat[i][j], (i, j));
        }
    }
    let mut row = vec![0; m];
    let mut col = vec![0; n];
    for (k, &v) in arr.iter().enumerate() {
        let &(i, j) = idx.get(&v).unwrap();
        row[i] += 1;
        col[j] += 1;
        if row[i] == n || col[j] == m {
            return k as i32;
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::first_complete_index;

    #[test]
    fn example_one() {
        assert_eq!(
            first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]),
            2
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            first_complete_index(
                vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
                vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]]
            ),
            3
        );
    }
}
