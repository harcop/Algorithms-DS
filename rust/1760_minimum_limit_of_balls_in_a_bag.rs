/// LeetCode #1760 - Minimum Limit of Balls in a Bag
fn minimum_size(nums: Vec<i32>, max_ops: i32) -> i32 {
    let mut lo = 1i32;
    let mut hi = *nums.iter().max().unwrap();
    while lo < hi {
        let mid = (lo + hi) / 2;
        let mut ops = 0i64;
        for &x in &nums {
            ops += (x as i64 - 1) / mid as i64;
            if ops > max_ops as i64 {
                break;
            }
        }
        if ops <= max_ops as i64 {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}
fn main() { println!("{}", minimum_size(vec![2, 4, 8, 2], 4)); }
#[cfg(test)]
mod tests {
    use super::minimum_size;
    #[test]
    fn example_one() { assert_eq!(minimum_size(vec![2, 4, 8, 2], 4), 2); }
    #[test]
    fn example_two() { assert_eq!(minimum_size(vec![3, 6, 7, 11], 5), 4); }
}
