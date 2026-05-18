/// LeetCode #973 - K Closest Points to Origin
fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut pts = points;
    pts.sort_by_key(|p| p[0] * p[0] + p[1] * p[1]);
    pts.truncate(k as usize);
    pts
}

fn main() {
    println!("{:?}", k_closest(vec![vec![1, 3], vec![-2, 2]], 1));
}

#[cfg(test)]
mod tests {
    use super::k_closest;

    #[test]
    fn example_one() {
        let mut out = k_closest(vec![vec![1, 3], vec![-2, 2]], 1);
        out.sort_by_key(|p| (p[0], p[1]));
        assert_eq!(out, vec![vec![-2, 2]]);
    }

    #[test]
    fn example_two() {
        let out = k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
        assert_eq!(out.len(), 2);
        let mut pts = out;
        pts.sort_by_key(|p| (p[0] * p[0] + p[1] * p[1], p[0], p[1]));
        assert_eq!(pts, vec![vec![3, 3], vec![-2, 4]]);
    }
}
