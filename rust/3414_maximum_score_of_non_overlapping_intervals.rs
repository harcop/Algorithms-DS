/// LeetCode #3414 - Maximum Score of Non-overlapping Intervals
fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let n = intervals.len();
    let mut ivs: Vec<(i32, i32, i32, i32)> = intervals
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v[0], v[1], v[2], i as i32))
        .collect();
    ivs.sort_unstable();
    let better = |a: &(i64, Vec<i32>), b: &(i64, Vec<i32>)| -> (i64, Vec<i32>) {
        if a.0 > b.0 || (a.0 == b.0 && a.1 < b.1) {
            a.clone()
        } else {
            b.clone()
        }
    };
    let empty = (0i64, Vec::new());
    let mut dp = vec![vec![empty.clone(); 5]; n + 1];
    for i in (0..n).rev() {
        let (l, r, w, idx) = ivs[i];
        let _ = l;
        let mut lo = 0;
        let mut hi = n;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if ivs[mid].0 > r {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        let j = lo;
        for q in 0..=4 {
            if q == 0 {
                dp[i][q] = empty.clone();
                continue;
            }
            let skip = &dp[i + 1][q];
            let next = &dp[j][q - 1];
            let mut pick_idx = next.1.clone();
            pick_idx.push(idx);
            pick_idx.sort_unstable();
            let pick = (next.0 + w as i64, pick_idx);
            dp[i][q] = better(skip, &pick);
        }
    }
    dp[0][4].1.clone()
}

fn main() {
    println!(
        "{:?}",
        maximum_weight(vec![
            vec![1, 3, 2],
            vec![4, 5, 2],
            vec![1, 5, 5],
            vec![6, 9, 3],
            vec![6, 7, 1],
            vec![8, 9, 1]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_weight;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_weight(vec![
                vec![1, 3, 2],
                vec![4, 5, 2],
                vec![1, 5, 5],
                vec![6, 9, 3],
                vec![6, 7, 1],
                vec![8, 9, 1]
            ]),
            vec![2, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_weight(vec![
                vec![5, 8, 1],
                vec![6, 7, 7],
                vec![4, 7, 3],
                vec![9, 10, 6],
                vec![7, 8, 2],
                vec![11, 14, 3],
                vec![3, 5, 5]
            ]),
            vec![1, 3, 5, 6]
        );
    }
}
