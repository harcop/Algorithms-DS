/// LeetCode #812 - Largest Triangle Area
fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut ans = 0.0f64;
    let n = points.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1, y1) = (points[i][0] as f64, points[i][1] as f64);
                let (x2, y2) = (points[j][0] as f64, points[j][1] as f64);
                let (x3, y3) = (points[k][0] as f64, points[k][1] as f64);
                let area = ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs();
                ans = ans.max(area);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", largest_triangle_area(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2]]));
}

#[cfg(test)]
mod tests {
    use super::largest_triangle_area;

    #[test]
    fn example_one() {
        let v = largest_triangle_area(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2]]);
        assert!(v > 0.0);
    }
}
