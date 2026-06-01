/// LeetCode #1643 - Kth Smallest Instructions
fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
    let (mut r, mut c) = (destination[0], destination[1]);
    let mut k = k as i64;
    let mut ans = String::new();
    fn comb(n: i64, k: i64) -> i64 {
        if k < 0 || k > n { return 0; }
        if k == 0 || k == n { return 1; }
        let k = k.min(n - k);
        let mut num = 1i64;
        let mut den = 1i64;
        for i in 0..k {
            num *= n - i;
            den *= i + 1;
        }
        num / den
    }
    while r > 0 || c > 0 {
        let right = comb((r + c - 1) as i64, (c - 1) as i64);
        if k <= right {
            ans.push('H');
            c -= 1;
        } else {
            ans.push('V');
            k -= right;
            r -= 1;
        }
    }
    ans
}
fn main() { println!("{}", kth_smallest_path(vec![2,3], 1)); }
#[cfg(test)]
mod tests {
    use super::kth_smallest_path;
    #[test]
    fn example_one() { assert_eq!(kth_smallest_path(vec![2,3], 1), "HHHVV"); }
    #[test]
    fn example_two() { assert_eq!(kth_smallest_path(vec![2,3], 2), "HHVHV"); }
}