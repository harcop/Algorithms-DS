/// LeetCode #1459 - Rectangles Area (SQL; Rust analogue)
fn rectangles_area(points: Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut ans = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (id1, x1, y1) = points[i];
            let (id2, x2, y2) = points[j];
            if x1 != x2 && y1 != y2 {
                let area = (x1 - x2).abs() * (y1 - y2).abs();
                let (p1, p2) = if id1 < id2 { (id1, id2) } else { (id2, id1) };
                ans.push((p1, p2, area));
            }
        }
    }
    ans.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)).then(a.1.cmp(&b.1)));
    ans
}

fn main() {
    println!("{:?}", rectangles_area(vec![]));
}

#[cfg(test)]
mod tests {
    use super::rectangles_area;

    #[test]
    fn example() {
        let points = vec![(1, 2, 7), (2, 4, 8), (3, 2, 10)];
        assert_eq!(rectangles_area(points), vec![(2, 3, 4), (1, 2, 2)]);
    }
}
