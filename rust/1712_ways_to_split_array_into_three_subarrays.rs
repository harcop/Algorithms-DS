/// LeetCode #1712 - Ways To Split Array Into Three Subarrays
const MOD: i64 = 1_000_000_007;

fn ways_to_split(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut pref = vec![0i64; n + 1];
    for i in 0..n { pref[i + 1] = pref[i] + nums[i] as i64; }
    let total = pref[n];
    let mut ans = 0i64;
    for i in 0..n - 2 {
        let left = pref[i + 1];
        if left * 2 > total { break; }
        let lo = i + 2;
        let hi = n - 1;
        let mut l = lo;
        let mut r = hi;
        while l <= r {
            let mid = (l + r) / 2;
            if pref[mid] - left >= left { r = mid - 1; } else { l = mid + 1; }
        }
        let start = l;
        l = lo; r = hi;
        while l <= r {
            let mid = (l + r) / 2;
            if pref[mid] - left <= total - pref[mid] { l = mid + 1; } else { r = mid - 1; }
        }
        let end = r;
        if start <= end { ans += (end - start + 1) as i64; }
    }
    (ans % MOD) as i32
}
fn main() { println!("{}", ways_to_split(vec![1,1,1])); }
#[cfg(test)]
mod tests {
    use super::ways_to_split;
    #[test]
    fn example_one() { assert_eq!(ways_to_split(vec![1,1,1]), 1); }
}