/// LeetCode #4001 - Aggregate Two Time Series
fn aggregate_time_series(series1: Vec<Vec<i32>>, series2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i = 0usize;
    let mut j = 0usize;
    let mut ans = Vec::new();
    while i < series1.len() && j < series2.len() {
        let (t1, v1) = (series1[i][0], series1[i][1]);
        let (t2, v2) = (series2[j][0], series2[j][1]);
        if t1 == t2 {
            ans.push(vec![t1, v1 + v2]);
            i += 1;
            j += 1;
        } else if t1 < t2 {
            ans.push(vec![t1, v1 + v2]);
            i += 1;
        } else {
            ans.push(vec![t2, v1 + v2]);
            j += 1;
        }
    }
    while i < series1.len() {
        ans.push(series1[i].clone());
        i += 1;
    }
    while j < series2.len() {
        ans.push(series2[j].clone());
        j += 1;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        aggregate_time_series(vec![vec![1, 3], vec![4, 1]], vec![vec![2, 2], vec![5, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::aggregate_time_series;

    #[test]
    fn example1() {
        assert_eq!(
            aggregate_time_series(vec![vec![1, 3], vec![4, 1]], vec![vec![2, 2], vec![5, 2]]),
            vec![vec![1, 5], vec![2, 3], vec![4, 3], vec![5, 2]]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            aggregate_time_series(vec![vec![1, 5], vec![3, 1]], vec![vec![2, 2]]),
            vec![vec![1, 7], vec![2, 3], vec![3, 1]]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            aggregate_time_series(vec![vec![1, 5]], vec![vec![1_000_000_000, 2]]),
            vec![vec![1, 7], vec![1_000_000_000, 2]]
        );
    }
}
