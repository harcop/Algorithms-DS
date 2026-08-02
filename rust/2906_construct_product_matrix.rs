/// LeetCode #2906 - Construct Product Matrix
fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut p = vec![vec![0; m]; n];
    const MOD: i64 = 12345;

    let mut suf = 1i64;
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            p[i][j] = suf as i32;
            suf = suf * grid[i][j] as i64 % MOD;
        }
    }

    let mut pre = 1i64;
    for i in 0..n {
        for j in 0..m {
            p[i][j] = (p[i][j] as i64 * pre % MOD) as i32;
            pre = pre * grid[i][j] as i64 % MOD;
        }
    }
    p
}

fn main() {
    println!("{:?}", construct_product_matrix(vec![vec![1, 2], vec![3, 4]]));
}

#[cfg(test)]
mod tests {
    use super::construct_product_matrix;

    #[test]
    fn example_one() {
        assert_eq!(
            construct_product_matrix(vec![vec![1, 2], vec![3, 4]]),
            vec![vec![24, 12], vec![8, 6]]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            construct_product_matrix(vec![vec![12345], vec![2], vec![1]]),
            vec![vec![2], vec![0], vec![0]]
        );
    }
}
