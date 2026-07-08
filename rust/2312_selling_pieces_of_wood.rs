/// LeetCode #2312 - Selling Pieces of Wood
fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
    let m = m as usize;
    let n = n as usize;
    let mut d = vec![vec![0i64; n + 1]; m + 1];
    for p in prices {
        d[p[0] as usize][p[1] as usize] = p[2] as i64;
    }
    let mut f = vec![vec![0i64; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            f[i][j] = d[i][j];
            for k in 1..i {
                f[i][j] = f[i][j].max(f[k][j] + f[i - k][j]);
            }
            for k in 1..j {
                f[i][j] = f[i][j].max(f[i][k] + f[i][j - k]);
            }
        }
    }
    f[m][n]
}

fn main() {
    println!(
        "{}",
        selling_wood(3, 5, vec![vec![1, 4, 2], vec![2, 2, 7], vec![2, 1, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::selling_wood;

    #[test]
    fn example_one() {
        assert_eq!(
            selling_wood(3, 5, vec![vec![1, 4, 2], vec![2, 2, 7], vec![2, 1, 3]]),
            19
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            selling_wood(4, 6, vec![vec![3, 2, 10], vec![1, 4, 2], vec![4, 1, 3]]),
            32
        );
    }
}
