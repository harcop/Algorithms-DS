/// LeetCode #793 - Preimage Size of Factorial Zeroes Function
fn preimage_size_fz(k: i32) -> i32 {
    fn f(x: i64) -> i64 {
        let mut cnt = 0i64;
        let mut p = 5i64;
        while p <= x {
            cnt += x / p;
            p *= 5;
        }
        cnt
    }
    let mut lo = 0i64;
    let mut hi = 5i64 * (k as i64 + 1);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if f(mid) < k as i64 {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let mut cnt = 0i32;
    while f(lo) == k as i64 {
        cnt += 1;
        lo += 1;
    }
    cnt
}

fn main() {
    println!("{}", preimage_size_fz(0));
}

#[cfg(test)]
mod tests {
    use super::preimage_size_fz;

    #[test]
    fn example_one() {
        assert_eq!(preimage_size_fz(0), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(preimage_size_fz(5), 0);
    }
}
