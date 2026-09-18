/// LeetCode #3796 - Find Maximum Value in a Constrained Sequence
fn max_value(n: i32, restrictions: Vec<Vec<i32>>, diff: Vec<i32>) -> i32 {
    let n = n as usize;
    const INF: i64 = 1_000_000_000_000;
    let mut cap = vec![INF; n];
    cap[0] = 0;
    for r in restrictions {
        let idx = r[0] as usize;
        cap[idx] = cap[idx].min(r[1] as i64);
    }
    for i in 1..n {
        cap[i] = cap[i].min(cap[i - 1] + diff[i - 1] as i64);
    }
    for i in (0..n - 1).rev() {
        cap[i] = cap[i].min(cap[i + 1] + diff[i] as i64);
    }
    *cap.iter().max().unwrap() as i32
}

fn main() {
    println!(
        "{}",
        max_value(
            10,
            vec![vec![3, 1], vec![8, 1]],
            vec![2, 2, 3, 1, 4, 5, 1, 1, 2]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_value;

    #[test]
    fn example1() {
        assert_eq!(
            max_value(
                10,
                vec![vec![3, 1], vec![8, 1]],
                vec![2, 2, 3, 1, 4, 5, 1, 1, 2]
            ),
            6
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_value(8, vec![vec![3, 2]], vec![3, 5, 2, 4, 2, 3, 1]),
            12
        );
    }
}
