/// LeetCode #1739 - Building Boxes
fn minimum_boxes(n: i32) -> i32 {
    let n = n as i64;
    let mut lo = 0i64;
    let mut hi = 1_000_000i64;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if mid * (mid + 1) / 2 <= n {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}
fn main() { println!("{}", minimum_boxes(10)); }
#[cfg(test)]
mod tests {
    use super::minimum_boxes;
    #[test]
    fn example_one() { assert_eq!(minimum_boxes(3), 2); }
    #[test]
    fn example_two() { assert_eq!(minimum_boxes(4), 2); }
    #[test]
    fn example_three() { assert_eq!(minimum_boxes(10), 4); }
}
