/// LeetCode #356 - Line Reflection (mirror about vertical axis x=(min+max)/2)
use std::collections::HashSet;

fn is_reflected(points: Vec<Vec<i32>>) -> bool {
    let mut mn = i32::MAX;
    let mut mx = i32::MIN;
    for p in &points {
        mn = mn.min(p[0]);
        mx = mx.max(p[0]);
    }
    let piv = mn as i64 + mx as i64;
    let set: HashSet<(i64, i32)> = points.iter().map(|p| (p[0] as i64, p[1])).collect();
    points
        .iter()
        .all(|p| set.contains(&(piv - p[0] as i64, p[1])))
}

fn main() {
    println!("{}", is_reflected(vec![vec![1, 1], vec![2, 2]]));
}

#[cfg(test)]
mod tests {
    use super::is_reflected;

    #[test]
    fn examples() {
        assert!(is_reflected(vec![vec![1, 1], vec![-1, 1]]));
        assert!(!is_reflected(vec![vec![1, 1], vec![2, 2]]));
    }
}
