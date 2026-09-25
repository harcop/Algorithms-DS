/// LeetCode #3938 - Maximum Path Intersection Sum in a Grid
fn max_subarray_at_least_two(a: &[i32]) -> i32 {
    let mut pref = 0i64;
    let mut min_pref = 0i64;
    let mut best = i64::MIN;
    for (i, &v) in a.iter().enumerate() {
        let prev = pref;
        pref += v as i64;
        if i >= 1 {
            best = best.max(pref - min_pref);
        }
        min_pref = min_pref.min(prev);
    }
    best as i32
}

fn max_intersection_sum(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = i32::MIN;
    for row in &grid {
        ans = ans.max(max_subarray_at_least_two(row));
    }
    for j in 0..n {
        let col: Vec<i32> = (0..m).map(|i| grid[i][j]).collect();
        ans = ans.max(max_subarray_at_least_two(&col));
    }
    if m >= 3 && n >= 3 {
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                ans = ans.max(grid[i][j]);
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        max_intersection_sum(vec![
            vec![1, 2, 0, -3],
            vec![1, -2, 1, 0],
            vec![-4, 2, -1, 3],
            vec![3, -3, 3, -2],
            vec![-1, -5, 0, 1],
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_intersection_sum;

    #[test]
    fn example1() {
        assert_eq!(
            max_intersection_sum(vec![
                vec![1, 2, 0, -3],
                vec![1, -2, 1, 0],
                vec![-4, 2, -1, 3],
                vec![3, -3, 3, -2],
                vec![-1, -5, 0, 1],
            ]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_intersection_sum(vec![vec![4, -2, -3], vec![-1, -3, -1], vec![-4, 2, -1]]),
            3
        );
    }
}
