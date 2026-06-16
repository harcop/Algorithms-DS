/// LeetCode #1924 - Erect the Fence II
#[derive(Clone, Copy)]
struct Circle {
    x: f64,
    y: f64,
    r: f64,
}

fn dist_sq(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx * dx + dy * dy
}

fn in_circle(c: Circle, x: f64, y: f64) -> bool {
    dist_sq(c.x, c.y, x, y) <= c.r * c.r + 1e-8
}

fn circle_two(p1: (f64, f64), p2: (f64, f64)) -> Circle {
    Circle {
        x: (p1.0 + p2.0) / 2.0,
        y: (p1.1 + p2.1) / 2.0,
        r: dist_sq(p1.0, p1.1, p2.0, p2.1).sqrt() / 2.0,
    }
}

fn circle_three(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> Circle {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));
    let ux = ((x1 * x1 + y1 * y1) * (y2 - y3)
        + (x2 * x2 + y2 * y2) * (y3 - y1)
        + (x3 * x3 + y3 * y3) * (y1 - y2))
        / d;
    let uy = ((x1 * x1 + y1 * y1) * (x3 - x2)
        + (x2 * x2 + y2 * y2) * (x1 - x3)
        + (x3 * x3 + y3 * y3) * (x2 - x1))
        / d;
    Circle {
        x: ux,
        y: uy,
        r: dist_sq(ux, uy, x1, y1).sqrt(),
    }
}

fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<f64> {
    let mut pts: Vec<(f64, f64)> = trees
        .iter()
        .map(|t| (t[0] as f64, t[1] as f64))
        .collect();
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut circle = Circle {
        x: pts[0].0,
        y: pts[0].1,
        r: 0.0,
    };

    for i in 0..pts.len() {
        let p = pts[i];
        if !in_circle(circle, p.0, p.1) {
            circle = Circle {
                x: p.0,
                y: p.1,
                r: 0.0,
            };
            for j in 0..i {
                let q = pts[j];
                if !in_circle(circle, q.0, q.1) {
                    circle = circle_two(p, q);
                    for k in 0..j {
                        let r = pts[k];
                        if !in_circle(circle, r.0, r.1) {
                            circle = circle_three(p, q, r);
                        }
                    }
                }
            }
        }
    }
    vec![circle.x, circle.y, circle.r]
}

fn main() {
    println!(
        "{:?}",
        outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::outer_trees;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-4
    }

    #[test]
    fn example_one() {
        let ans = outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ]);
        assert!(close(ans[0], 2.0));
        assert!(close(ans[1], 2.0));
        assert!(close(ans[2], 2.0));
    }
}
