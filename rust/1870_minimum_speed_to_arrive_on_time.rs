/// LeetCode #1870 - Minimum Speed to Arrive on Time
fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    let n = dist.len();
    if n as f64 > hour.ceil() {
        return -1;
    }
    let check = |v: i32| -> bool {
        let mut s = 0.0;
        for (i, &d) in dist.iter().enumerate() {
            let t = d as f64 / v as f64;
            s += if i == n - 1 { t } else { t.ceil() };
        }
        s <= hour
    };
    let mut lo = 1i32;
    let mut hi = 10_000_001i32;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if check(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    if lo == 10_000_001 {
        -1
    } else {
        lo
    }
}

fn main() {
    println!("{}", min_speed_on_time(vec![1, 3, 2], 6.0));
}

#[cfg(test)]
mod tests {
    use super::min_speed_on_time;

    #[test]
    fn example_one() {
        assert_eq!(min_speed_on_time(vec![1, 3, 2], 6.0), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_speed_on_time(vec![1, 3, 2], 2.7), 3);
    }
}
