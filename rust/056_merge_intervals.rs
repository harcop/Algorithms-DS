/// LeetCode #56 - Merge Intervals
fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }
    intervals.sort_unstable_by_key(|v| v[0]);
    let mut out = vec![intervals[0].clone()];

    for cur in intervals.into_iter().skip(1) {
        let last = out.last_mut().unwrap();
        if cur[0] <= last[1] {
            last[1] = last[1].max(cur[1]);
        } else {
            out.push(cur);
        }
    }
    out
}

fn main() {
    println!("{:?}", merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]));
}

#[cfg(test)]
mod tests {
    use super::merge;
    #[test]
    fn example_one() {
        assert_eq!(merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }
    #[test]
    fn example_two() {
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
    }
}
