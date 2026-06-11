/// LeetCode #1828 - Queries on Number of Points Inside a Circle
fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let (x, y, r) = (q[0], q[1], q[2]);
        let r2 = (r as i64) * (r as i64);
        let mut cnt = 0i32;
        for p in &points {
            let dx = p[0] as i64 - x as i64;
            let dy = p[1] as i64 - y as i64;
            if dx * dx + dy * dy <= r2 {
                cnt += 1;
            }
        }
        ans.push(cnt);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        count_points(
            vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
            vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_points;

    #[test]
    fn example_one() {
        assert_eq!(
            count_points(
                vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
                vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]],
            ),
            vec![3, 2, 2]
        );
    }
}
