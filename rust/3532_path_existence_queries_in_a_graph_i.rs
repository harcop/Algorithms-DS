/// LeetCode #3532 - Path Existence Queries in a Graph I
fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut g = vec![0; n];
    let mut cnt = 0;
    for i in 1..n {
        if nums[i] - nums[i - 1] > max_diff {
            cnt += 1;
        }
        g[i] = cnt;
    }
    queries.into_iter().map(|q| g[q[0] as usize] == g[q[1] as usize]).collect()
}

fn main() {
    println!(
        "{:?}",
        path_existence_queries(2, vec![1, 3], 1, vec![vec![0, 0], vec![0, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::path_existence_queries;

    #[test]
    fn example1() {
        assert_eq!(
            path_existence_queries(2, vec![1, 3], 1, vec![vec![0, 0], vec![0, 1]]),
            vec![true, false]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            path_existence_queries(
                4,
                vec![2, 5, 6, 8],
                2,
                vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![2, 3]]
            ),
            vec![false, false, true, true]
        );
    }
}
