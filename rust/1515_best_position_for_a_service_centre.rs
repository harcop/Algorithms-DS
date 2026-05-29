/// LeetCode #1515 - Best Position For A Service Centre
fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
    let pts: Vec<(f64, f64)> = positions.iter().map(|p| (p[0] as f64, p[1] as f64)).collect();
    let mut x = pts.iter().map(|p| p.0).sum::<f64>() / pts.len() as f64;
    let mut y = pts.iter().map(|p| p.1).sum::<f64>() / pts.len() as f64;
    for _ in 0..100 {
        let mut nx = 0.0;
        let mut ny = 0.0;
        let mut denom = 0.0;
        for &(px, py) in &pts {
            let d = ((px - x).hypot(py - y)).max(1e-7);
            nx += px / d;
            ny += py / d;
            denom += 1.0 / d;
        }
        let new_x = nx / denom;
        let new_y = ny / denom;
        if (new_x - x).hypot(new_y - y) < 1e-7 {
            break;
        }
        x = new_x;
        y = new_y;
    }
    pts.iter().map(|&(px, py)| (px - x).hypot(py - y)).sum()
}

fn main() {
    println!("{}", get_min_dist_sum(vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]]));
}

#[cfg(test)]
mod tests {
    use super::get_min_dist_sum;

    #[test]
    fn example_one() {
        let ans = get_min_dist_sum(vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]]);
        assert!((ans - 4.0).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        let ans = get_min_dist_sum(vec![vec![1, 1], vec![0, 0], vec![2, 0]]);
        assert!((ans - 2.82843).abs() < 0.15);
    }
}
