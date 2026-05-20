/// LeetCode #1179 - Find K-th Largest XOR Coordinate Value
fn kth_largest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
    let n = mat.len();
    let m = mat[0].len();
    let mut grid = vec![vec![0i32; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            grid[i + 1][j + 1] =
                mat[i][j] ^ grid[i][j + 1] ^ grid[i + 1][j] ^ grid[i + 1][j + 1];
        }
    }
    let mut vals = Vec::new();
    for i in 1..=n {
        for j in 1..=m {
            vals.push(grid[i][j]);
        }
    }
    let k = k as usize;
    vals.select_nth_unstable_by(k - 1, |a, b| b.cmp(a));
    vals[k - 1]
}

fn main() {
    let mat = vec![vec![5, 2], vec![1, 6]];
    println!("{}", kth_largest(mat, 1));
}

#[cfg(test)]
mod tests {
    use super::kth_largest;

    #[test]
    fn example_one() {
        assert_eq!(kth_largest(vec![vec![5, 2], vec![1, 6]], 1), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            kth_largest(vec![vec![5, 2], vec![1, 6]], 2),
            5
        );
    }
}
