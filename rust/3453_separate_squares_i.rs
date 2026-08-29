/// LeetCode #3453 - Separate Squares I
fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    fn area_below(squares: &[Vec<i32>], y1: f64) -> f64 {
        let mut t = 0.0;
        for sq in squares {
            let y = sq[1] as f64;
            let l = sq[2] as f64;
            if y < y1 {
                t += l * (y1 - y).min(l);
            }
        }
        t
    }

    let total: f64 = squares.iter().map(|a| (a[2] as f64) * (a[2] as f64)).sum();
    let mut lo = 0.0;
    let mut hi = squares
        .iter()
        .map(|a| (a[1] + a[2]) as f64)
        .fold(0.0, f64::max);
    for _ in 0..80 {
        let mid = (lo + hi) / 2.0;
        if area_below(&squares, mid) >= total / 2.0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    hi
}

fn main() {
    println!(
        "{}",
        separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::separate_squares;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-4, "{a} vs {b}");
    }

    #[test]
    fn example1() {
        close(separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]), 1.0);
    }

    #[test]
    fn example2() {
        close(
            separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]),
            7.0 / 6.0,
        );
    }
}
