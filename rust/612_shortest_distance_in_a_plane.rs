/// LeetCode #612 - Shortest Distance in a Plane (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn shortest_distance_plane(point: Vec<(i32, i32)>) -> f64 {
    let mut best = f64::MAX;
    for i in 0..point.len() {
        for j in i + 1..point.len() {
            let dx = (point[i].0 - point[j].0) as f64;
            let dy = (point[i].1 - point[j].1) as f64;
            best = best.min((dx * dx + dy * dy).sqrt());
        }
    }
    round2(best)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::shortest_distance_plane;

    #[test]
    fn example() {
        let point = vec![(-1, -1), (0, 0), (-1, -2)];
        assert!((shortest_distance_plane(point) - 1.0).abs() < 1e-9);
    }
}
