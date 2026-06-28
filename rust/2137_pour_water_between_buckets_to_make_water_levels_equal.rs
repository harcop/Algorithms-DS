/// LeetCode #2137 - Pour Water Between Buckets to Make Water Levels Equal
fn equalize_water(buckets: Vec<i32>, loss: i32) -> f64 {
    let keep = (100 - loss) as f64 / 100.0;
    let mut low = 0.0;
    let mut high = buckets.iter().copied().max().unwrap() as f64;

    for _ in 0..100 {
        let mid = (low + high) / 2.0;
        if can_reach(&buckets, keep, mid) {
            low = mid;
        } else {
            high = mid;
        }
    }

    low
}

fn can_reach(buckets: &[i32], keep: f64, level: f64) -> bool {
    let mut surplus = 0.0;
    let mut need = 0.0;

    for &bucket in buckets {
        let water = bucket as f64;
        if water > level {
            surplus += water - level;
        } else {
            need += level - water;
        }
    }

    surplus * keep >= need
}

fn main() {
    println!("{:.5}", equalize_water(vec![1, 2, 7], 80));
}

#[cfg(test)]
mod tests {
    use super::equalize_water;

    fn assert_close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1e-5, "{actual} != {expected}");
    }

    #[test]
    fn example_one() {
        assert_close(equalize_water(vec![1, 2, 7], 80), 2.0);
    }

    #[test]
    fn example_two() {
        assert_close(equalize_water(vec![2, 4, 6], 50), 3.5);
    }
}
