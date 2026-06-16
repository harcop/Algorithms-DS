/// LeetCode #1928 - Minimum Cost to Reach Destination in Time
const INF: i32 = i32::MAX / 2;

fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
    let m = max_time as usize;
    let n = passing_fees.len();
    let mut f = vec![vec![INF; n]; m + 1];
    f[0][0] = passing_fees[0];

    for i in 1..=m {
        for e in &edges {
            let x = e[0] as usize;
            let y = e[1] as usize;
            let t = e[2] as usize;
            if t <= i {
                f[i][x] = f[i][x].min(f[i - t][y] + passing_fees[x]);
                f[i][y] = f[i][y].min(f[i - t][x] + passing_fees[y]);
            }
        }
    }

    let ans = (0..=m).map(|i| f[i][n - 1]).min().unwrap_or(INF);
    if ans == INF { -1 } else { ans }
}

fn main() {
    println!(
        "{}",
        min_cost(
            30,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(
            min_cost(
                30,
                vec![
                    vec![0, 1, 10],
                    vec![1, 2, 10],
                    vec![2, 5, 10],
                    vec![0, 3, 1],
                    vec![3, 4, 10],
                    vec![4, 5, 15],
                ],
                vec![5, 1, 2, 20, 20, 3]
            ),
            11
        );
    }
}
