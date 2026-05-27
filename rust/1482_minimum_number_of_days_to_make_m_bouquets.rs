/// LeetCode #1482 - Minimum Number Of Days To Make M Bouquets
fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
    let n = bloom_day.len();
    if (m as usize).saturating_mul(k as usize) > n { return -1; }
    let mut lo = *bloom_day.iter().min().unwrap();
    let mut hi = *bloom_day.iter().max().unwrap();
    let can = |days: i32| -> bool {
        let mut bouquets = 0i32;
        let mut flowers = 0i32;
        for &d in &bloom_day {
            if d <= days {
                flowers += 1;
                if flowers == k { bouquets += 1; flowers = 0; }
            } else { flowers = 0; }
        }
        bouquets >= m
    };
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if can(mid) { hi = mid; } else { lo = mid + 1; }
    }
    lo
}
fn main() { println!("{}", min_days(vec![1,10,3,10,2], 3, 1)); }
#[cfg(test)]
mod tests {
    use super::min_days;
    #[test]
    fn example_one() { assert_eq!(min_days(vec![1,10,3,10,2], 3, 1), 3); }
    #[test]
    fn example_two() { assert_eq!(min_days(vec![1,10,3,10,2], 3, 2), -1); }
    #[test]
    fn example_three() { assert_eq!(min_days(vec![7,7,7,7,12,7,7], 2, 3), 12); }
}