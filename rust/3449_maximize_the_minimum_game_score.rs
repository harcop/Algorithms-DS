/// LeetCode #3449 - Maximize the Minimum Game Score
fn max_score(points: Vec<i32>, m: i32) -> i64 {
    fn check(points: &[i32], x: i64, m: i64) -> bool {
        let mut cnt = 0i64;
        let mut prev = 0i64;
        for (i, &p) in points.iter().enumerate() {
            let need = (x + p as i64 - 1) / p as i64 - prev;
            if need >= 1 {
                prev = need - 1;
                cnt += 2 * need - 1;
            } else if i + 1 != points.len() {
                prev = 0;
                cnt += 1;
            }
            if cnt > m {
                return false;
            }
        }
        true
    }
    let m = m as i64;
    let mx = *points.iter().max().unwrap() as i64;
    let mut lo = 0i64;
    let mut hi = mx * m;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if check(&points, mid, m) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

fn main() {
    println!("{}", max_score(vec![2, 4], 3));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![2, 4], 3), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![1, 2, 3], 5), 2);
    }
}
