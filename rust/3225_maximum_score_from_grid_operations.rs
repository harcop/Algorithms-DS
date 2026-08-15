/// LeetCode #3225 - Maximum Score From Grid Operations
fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid.len();
    let mut prefix = vec![vec![0i64; n + 1]; n];
    for j in 0..n {
        for i in 0..n {
            prefix[j][i + 1] = prefix[j][i] + grid[i][j] as i64;
        }
    }
    let mut prev_pick = vec![0i64; n + 1];
    let mut prev_skip = vec![0i64; n + 1];
    for j in 1..n {
        let mut curr_pick = vec![0i64; n + 1];
        let mut curr_skip = vec![0i64; n + 1];
        for curr in 0..=n {
            for prev in 0..=n {
                if curr > prev {
                    let score = prefix[j - 1][curr] - prefix[j - 1][prev];
                    curr_pick[curr] = curr_pick[curr].max(prev_skip[prev] + score);
                    curr_skip[curr] = curr_skip[curr].max(prev_skip[prev] + score);
                } else {
                    let score = prefix[j][prev] - prefix[j][curr];
                    curr_pick[curr] = curr_pick[curr].max(prev_pick[prev] + score);
                    curr_skip[curr] = curr_skip[curr].max(prev_pick[prev]);
                }
            }
        }
        prev_pick = curr_pick;
        prev_skip = curr_skip;
    }
    *prev_pick.iter().max().unwrap()
}

fn main() {
    println!(
        "{}",
        maximum_score(vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 3, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![5, 0, 0, 3, 0],
            vec![0, 0, 0, 0, 2]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_score;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_score(vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 3, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![5, 0, 0, 3, 0],
                vec![0, 0, 0, 0, 2]
            ]),
            11
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_score(vec![
                vec![10, 9, 0, 0, 15],
                vec![7, 1, 0, 8, 0],
                vec![5, 20, 0, 11, 0],
                vec![0, 0, 0, 1, 2],
                vec![8, 12, 1, 10, 3]
            ]),
            94
        );
    }
}
