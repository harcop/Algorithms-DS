/// LeetCode #469 - Convex Polygon
fn is_convex(points: Vec<Vec<i32>>) -> bool {
    let n = points.len();
    let mut prev = 0i64;
    for i in 0..n {
        let ax = points[i][0] as i64;
        let ay = points[i][1] as i64;
        let bx = points[(i + 1) % n][0] as i64;
        let by = points[(i + 1) % n][1] as i64;
        let cx = points[(i + 2) % n][0] as i64;
        let cy = points[(i + 2) % n][1] as i64;
        let cross = (bx - ax) * (cy - by) - (by - ay) * (cx - bx);
        if cross != 0 {
            if cross * prev < 0 {
                return false;
            }
            prev = cross;
        }
    }
    true
}

fn main() {
    println!(
        "{}",
        is_convex(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 1],
            vec![1, 0]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::is_convex;

    #[test]
    fn example_one() {
        assert!(is_convex(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 1],
            vec![1, 0]
        ]));
    }

    #[test]
    fn example_two() {
        assert!(!is_convex(vec![
            vec![0, 0],
            vec![0, 10],
            vec![10, 10],
            vec![10, 0],
            vec![5, 5]
        ]));
    }
}
