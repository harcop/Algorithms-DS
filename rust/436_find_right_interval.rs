/// LeetCode #436 - Find Right Interval
fn find_right_interval(mut intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let n = intervals.len();
    let mut idxs: Vec<(i32, usize)> = intervals.iter().enumerate().map(|(i, v)| (v[0], i)).collect();
    idxs.sort_by_key(|x| x.0);
    let mut ans = vec![-1i32; n];
    for i in 0..n {
        let end = intervals[i][1];
        let mut lo = 0usize;
        let mut hi = idxs.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if idxs[mid].0 >= end {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if lo < idxs.len() {
            ans[i] = idxs[lo].1 as i32;
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_right_interval(vec![vec![1, 2]]));
}

#[cfg(test)]
mod tests {
    use super::find_right_interval;

    #[test]
    fn example_one() {
        assert_eq!(find_right_interval(vec![vec![1, 2]]), vec![-1]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
            vec![-1, 0, 1]
        );
    }
}
