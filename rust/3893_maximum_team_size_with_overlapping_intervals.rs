/// LeetCode #3893 - Maximum Team Size With Overlapping Intervals

fn maximum_team_size(start: Vec<i32>, end: Vec<i32>) -> i32 {
    let n = start.len();
    let mut starts = start.clone();
    let mut ends = end.clone();
    starts.sort_unstable();
    ends.sort_unstable();

    let mut best = 0;
    for i in 0..n {
        let l = start[i];
        let r = end[i];
        let hi = starts.partition_point(|&x| x <= r);
        let lo = ends.partition_point(|&x| x <= l - 1);
        best = best.max(hi - lo);
    }
    best as i32
}

fn main() {
    println!(
        "{}",
        maximum_team_size(vec![1, 2, 3], vec![4, 5, 6])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_team_size;

    #[test]
    fn example1() {
        assert_eq!(maximum_team_size(vec![1, 2, 3], vec![4, 5, 6]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_team_size(vec![2, 5, 8], vec![3, 7, 9]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_team_size(vec![3, 4, 6], vec![8, 5, 7]), 3);
    }
}
