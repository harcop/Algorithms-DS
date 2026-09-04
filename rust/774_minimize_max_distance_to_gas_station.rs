/// LeetCode #774 - Minimize Max Distance to Gas Station
fn minmax_gas_dist(stations: Vec<i32>, k: i32) -> f64 {
    let mut lo = 0.0f64;
    let mut hi = 0.0f64;
    for w in stations.windows(2) {
        hi = hi.max((w[1] - w[0]) as f64);
    }
    for _ in 0..80 {
        let mid = (lo + hi) / 2.0;
        if can_place(&stations, k, mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

fn can_place(stations: &[i32], k: i32, d: f64) -> bool {
    let mut need = 0i32;
    for w in stations.windows(2) {
        let gap = (w[1] - w[0]) as f64;
        need += ((gap / d).ceil() as i32) - 1;
        if need > k {
            return false;
        }
    }
    need <= k
}

fn main() {
    println!("{}", minmax_gas_dist(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9));
}

#[cfg(test)]
mod tests {
    use super::minmax_gas_dist;

    #[test]
    fn example_one() {
        let ans = minmax_gas_dist(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9);
        assert!((ans - 0.5).abs() < 1e-6);
    }

    #[test]
    fn example_two() {
        let ans = minmax_gas_dist(vec![1, 5], 2);
        assert!((ans - 4.0 / 3.0).abs() < 1e-6);
    }
}
