/// LeetCode #1851 - Minimum Interval to Include Each Query
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    let mut intervals = intervals;
    intervals.sort_by_key(|x| x[0]);
    let m = queries.len();
    let mut qs: Vec<(i32, usize)> = queries
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();
    qs.sort_by_key(|&(x, _)| x);

    let mut ans = vec![-1; m];
    let mut pq: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
    let mut i = 0usize;
    let n = intervals.len();

    for (x, j) in qs {
        while i < n && intervals[i][0] <= x {
            let a = intervals[i][0];
            let b = intervals[i][1];
            pq.push(Reverse((b - a + 1, b)));
            i += 1;
        }
        while let Some(Reverse((_, end))) = pq.peek().copied() {
            if end < x {
                pq.pop();
            } else {
                break;
            }
        }
        if let Some(Reverse((size, _))) = pq.peek() {
            ans[j] = *size;
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        min_interval(
            vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]],
            vec![2, 3, 4, 5],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::min_interval;

    #[test]
    fn example_one() {
        assert_eq!(
            min_interval(
                vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]],
                vec![2, 3, 4, 5],
            ),
            vec![3, 3, 1, 4]
        );
    }
}
