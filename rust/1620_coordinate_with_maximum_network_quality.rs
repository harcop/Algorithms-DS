/// LeetCode #1620 - Coordinate With Maximum Network Quality
fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
    let mut best = vec![0, 0];
    let mut best_q = -1i32;
    let r2 = radius as i64 * radius as i64;
    for x in 0..=100 {
        for y in 0..=100 {
            let mut q = 0i32;
            for t in &towers {
                let dx = x - t[0];
                let dy = y - t[1];
                let d2 = dx as i64 * dx as i64 + dy as i64 * dy as i64;
                if d2 <= r2 {
                    let d = ((d2 as f64).sqrt()) as i32;
                    q += t[2] / (1 + d);
                }
            }
            if q > best_q || (q == best_q && (x < best[0] || (x == best[0] && y < best[1]))) {
                best_q = q;
                best = vec![x, y];
            }
        }
    }
    best
}

fn main() {
    println!("{:?}", best_coordinate(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2));
}

#[cfg(test)]
mod tests {
    use super::best_coordinate;

    #[test]
    fn example_one() {
        assert_eq!(
            best_coordinate(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2),
            vec![2, 1]
        );
    }
}
