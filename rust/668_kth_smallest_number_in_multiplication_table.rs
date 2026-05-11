/// LeetCode #668 - Kth Smallest Number in Multiplication Table
fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    let mut lo = 1i32;
    let mut hi = m * n;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let mut count = 0i32;
        for i in 1..=m {
            count += (mid / i).min(n);
        }
        if count < k { lo = mid + 1; } else { hi = mid; }
    }
    lo
}

fn main() {
    println!("{}", find_kth_number(3, 3, 5));
}

#[cfg(test)]
mod tests {
    use super::find_kth_number;

    #[test]
    fn example_one() {
        assert_eq!(find_kth_number(3, 3, 5), 3);
    }
}
