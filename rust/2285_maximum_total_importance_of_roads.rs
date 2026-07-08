/// LeetCode #2285 - Maximum Total Importance of Roads
fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut deg = vec![0i64; n];
    for e in &roads {
        deg[e[0] as usize] += 1;
        deg[e[1] as usize] += 1;
    }

    let mut ids: Vec<usize> = (0..n).collect();
    ids.sort_by_key(|&i| deg[i]);

    let mut value = vec![0i64; n];
    for (rank, &i) in ids.iter().enumerate() {
        value[i] = (rank as i64) + 1;
    }

    let mut ans = 0i64;
    for e in roads {
        ans += value[e[0] as usize] + value[e[1] as usize];
    }
    ans
}

fn main() {
    println!(
        "{}",
        maximum_importance(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 2], vec![1, 3], vec![2, 4]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_importance;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_importance(
                5,
                vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 2], vec![1, 3], vec![2, 4]]
            ),
            43
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_importance(5, vec![vec![0, 3], vec![2, 4], vec![1, 3]]),
            20
        );
    }
}

