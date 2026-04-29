/// LeetCode #57 - Insert Interval
fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let (mut s, mut e) = (new_interval[0], new_interval[1]);
    let mut out = Vec::new();
    let mut i = 0usize;

    while i < intervals.len() && intervals[i][1] < s {
        out.push(intervals[i].clone());
        i += 1;
    }
    while i < intervals.len() && intervals[i][0] <= e {
        s = s.min(intervals[i][0]);
        e = e.max(intervals[i][1]);
        i += 1;
    }
    out.push(vec![s, e]);
    while i < intervals.len() {
        out.push(intervals[i].clone());
        i += 1;
    }
    out
}

fn main() {
    println!("{:?}", insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]));
}

#[cfg(test)]
mod tests {
    use super::insert;
    #[test]
    fn example_one() {
        assert_eq!(insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]), vec![vec![1, 5], vec![6, 9]]);
    }
    #[test]
    fn example_two() {
        assert_eq!(insert(vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]], vec![4, 8]), vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }
}
