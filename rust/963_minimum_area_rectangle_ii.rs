/// LeetCode #963 - Minimum Area Rectangle II

fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
    use std::collections::HashMap;
    let n = points.len();
    let mut seen: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
    for (i, p) in points.iter().enumerate() {
        seen.entry((p[0] as i64, p[1] as i64)).or_default().push(i);
    }
    let mut ans = f64::MAX;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1, y1) = (points[i][0] as i64, points[i][1] as i64);
                let (x2, y2) = (points[j][0] as i64, points[j][1] as i64);
                let (x3, y3) = (points[k][0] as i64, points[k][1] as i64);
                let dx1 = x2 - x1;
                let dy1 = y2 - y1;
                let dx2 = x3 - x1;
                let dy2 = y3 - y1;
                if dx1 * dx2 + dy1 * dy2 != 0 {
                    continue;
                }
                let x4 = x2 + x3 - x1;
                let y4 = y2 + y3 - y1;
                if seen.contains_key(&(x4, y4)) {
                    let area = ((dx1 * dx1 + dy1 * dy1) as f64).sqrt()
                        * ((dx2 * dx2 + dy2 * dy2) as f64).sqrt();
                    ans = ans.min(area);
                }
            }
        }
    }
    if ans == f64::MAX { 0.0 } else { ans }
}

fn main() {
    println!("{}", min_area_free_rect(vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]]));
}

#[cfg(test)]
mod tests {
    use super::min_area_free_rect;

    #[test]
    fn example_one() {
        let a = min_area_free_rect(vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]]);
        assert!((a - 2.0).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        let a = min_area_free_rect(vec![vec![0, 1], vec![2, 1], vec![1, 1], vec![1, 0], vec![2, 0]]);
        assert!((a - 1.0).abs() < 1e-5);
    }
}
