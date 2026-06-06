/// LeetCode #1779 - Find Nearest Point That Has the Same X or Y Coordinate
fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let mut best = i32::MAX;
    let mut ans = -1;
    for (i, p) in points.iter().enumerate() {
        if p[0] == x || p[1] == y {
            let d = (p[0] - x).abs() + (p[1] - y).abs();
            if d < best {
                best = d;
                ans = i as i32;
            }
        }
    }
    ans
}
fn main() {
    println!(
        "{}",
        nearest_valid_point(3, 4, vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]])
    );
}
#[cfg(test)]
mod tests {
    use super::nearest_valid_point;
    #[test]
    fn example_one() {
        assert_eq!(
            nearest_valid_point(3, 4, vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]),
            2
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(nearest_valid_point(3, 4, vec![vec![3, 4]]), 0);
    }
    #[test]
    fn example_three() {
        assert_eq!(nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
    }
}
