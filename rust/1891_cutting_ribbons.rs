/// LeetCode #1891 - Cutting Ribbons
fn max_length(ribbons: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let mut lo = 0i64;
    let mut hi = *ribbons.iter().max().unwrap_or(&0) as i64;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        let cnt: i64 = ribbons.iter().map(|&x| x as i64 / mid).sum();
        if cnt >= k {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn main() {
    println!("{}", max_length(vec![9, 7, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::max_length;

    #[test]
    fn example_one() {
        assert_eq!(max_length(vec![9, 7, 5], 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_length(vec![7, 5, 9], 4), 4);
    }
}
