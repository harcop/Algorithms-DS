/// LeetCode #3733 - Minimum Time to Complete All Deliveries
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn min_time(d: Vec<i32>, r: Vec<i32>) -> i64 {
    let d1 = d[0] as i64;
    let d2 = d[1] as i64;
    let r1 = r[0] as i64;
    let r2 = r[1] as i64;
    let l = r1 / gcd(r1, r2) * r2;
    let ok = |t: i64| {
        let a1 = t - t / r1;
        let a2 = t - t / r2;
        let shared = t - t / r1 - t / r2 + t / l;
        a1 >= d1 && a2 >= d2 && a1 + a2 - shared >= d1 + d2
    };
    let mut lo = d1 + d2;
    let mut hi = (d1 + d2) * 4;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if ok(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main() {
    println!("{}", min_time(vec![3, 1], vec![2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_time;

    #[test]
    fn example1() {
        assert_eq!(min_time(vec![3, 1], vec![2, 3]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(min_time(vec![1, 3], vec![2, 2]), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(min_time(vec![2, 1], vec![3, 4]), 3);
    }
}
