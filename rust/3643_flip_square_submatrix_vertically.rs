/// LeetCode #3643 - Flip Square Submatrix Vertically
fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let x = x as usize;
    let y = y as usize;
    let k = k as usize;
    for i in x..(x + k / 2) {
        let i2 = x + k - 1 - (i - x);
        for j in y..(y + k) {
            let t = grid[i][j];
            grid[i][j] = grid[i2][j];
            grid[i2][j] = t;
        }
    }
    grid
}

fn main() {
    println!("{:?}", reverse_submatrix(
        vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16]],
        1, 0, 3
    ));
}

#[cfg(test)]
mod tests {
    use super::reverse_submatrix;

    #[test]
    fn example1() {
        assert_eq!(
            reverse_submatrix(
                vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12],vec![13,14,15,16]],
                1, 0, 3
            ),
            vec![vec![1,2,3,4],vec![13,14,15,8],vec![9,10,11,12],vec![5,6,7,16]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            reverse_submatrix(vec![vec![3,4,2,3],vec![2,3,4,2]], 0, 2, 2),
            vec![vec![3,4,4,2],vec![2,3,2,3]]
        );
    }
}
