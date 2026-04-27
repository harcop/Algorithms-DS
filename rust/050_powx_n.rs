/// LeetCode #50 - Pow(x, n)
fn my_pow(x: f64, n: i32) -> f64 {
    fn fast_pow(x: f64, n: i64) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let half = fast_pow(x, n / 2);
        if n % 2 == 0 {
            half * half
        } else {
            half * half * x
        }
    }

    let n64 = n as i64;
    if n64 >= 0 {
        fast_pow(x, n64)
    } else {
        1.0 / fast_pow(x, -n64)
    }
}

fn main() {
    println!("{}", my_pow(2.0, 10));
}

#[cfg(test)]
mod tests {
    use super::my_pow;

    #[test]
    fn example_one() {
        assert!((my_pow(2.0, 10) - 1024.0).abs() < 1e-9);
    }

    #[test]
    fn example_two() {
        assert!((my_pow(2.1, 3) - 9.261).abs() < 1e-6);
    }

    #[test]
    fn example_three() {
        assert!((my_pow(2.0, -2) - 0.25).abs() < 1e-9);
    }
}
