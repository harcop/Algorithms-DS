/// LeetCode #3296 - Minimum Number of Seconds to Make Mountain Height Zero
fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
    let h0 = mountain_height as i64;
    let height_for = |wt: i64, t: i64| -> i64 {
        let mut lo = 0i64;
        let mut hi = h0;
        while lo < hi {
            let mid = (lo + hi + 1) >> 1;
            if wt.saturating_mul(mid).saturating_mul(mid + 1) / 2 <= t {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    };
    let check = |t: i64| -> bool {
        let mut h = 0i64;
        for &wt in &worker_times {
            h += height_for(wt as i64, t);
            if h >= h0 {
                return true;
            }
        }
        h >= h0
    };
    let mut l = 1i64;
    let mut r = 10i64.pow(16);
    while l < r {
        let mid = (l + r) >> 1;
        if check(mid) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    l
}

fn main() {
    println!("{}", min_number_of_seconds(4, vec![2, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_number_of_seconds;

    #[test]
    fn example1() {
        assert_eq!(min_number_of_seconds(4, vec![2, 1, 1]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_number_of_seconds(10, vec![3, 2, 2, 4]), 12);
    }

    #[test]
    fn example3() {
        assert_eq!(min_number_of_seconds(5, vec![1]), 15);
    }
}
