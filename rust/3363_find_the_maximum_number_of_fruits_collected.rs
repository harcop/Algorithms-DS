/// LeetCode #3363 - Find the Maximum Number of Fruits Collected
fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
    const NEG: i32 = i32::MIN / 4;
    let n = fruits.len();
    let mut f = vec![vec![NEG; n]; n];
    f[0][n - 1] = fruits[0][n - 1];
    for i in 1..n {
        for j in (i + 1)..n {
            let mut best = f[i - 1][j].max(f[i - 1][j - 1]);
            if j + 1 < n {
                best = best.max(f[i - 1][j + 1]);
            }
            f[i][j] = best + fruits[i][j];
        }
    }
    f[n - 1][0] = fruits[n - 1][0];
    for j in 1..n {
        for i in (j + 1)..n {
            let mut best = f[i][j - 1].max(f[i - 1][j - 1]);
            if i + 1 < n {
                best = best.max(f[i + 1][j - 1]);
            }
            f[i][j] = best + fruits[i][j];
        }
    }
    let diag: i32 = (0..n).map(|i| fruits[i][i]).sum();
    diag + f[n - 2][n - 1] + f[n - 1][n - 2]
}

fn main() {
    println!(
        "{}",
        max_collected_fruits(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 8, 7],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_collected_fruits;

    #[test]
    fn example1() {
        assert_eq!(
            max_collected_fruits(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 8, 7],
                vec![9, 10, 11, 12],
                vec![13, 14, 15, 16],
            ]),
            100
        );
    }

    #[test]
    fn example2() {
        assert_eq!(max_collected_fruits(vec![vec![1, 1], vec![1, 1]]), 4);
    }
}
