/// LeetCode #1590 - Make Sum Divisible By P
fn min_subarray_len(nums: Vec<i32>, p: i32) -> i32 {
    let total: i64 = nums.iter().map(|&x| x as i64).sum();
    let p = p as i64;
    let rem = ((total % p) + p) % p;
    if rem == 0 { return 0; }
    let need = rem;
    use std::collections::HashMap;
    let mut best = (nums.len() + 1) as i32;
    let mut cur = 0i64;
    let mut mp = HashMap::new();
    mp.insert(0i64, -1i32);
    for (i, &x) in nums.iter().enumerate() {
        cur = (cur + x as i64) % p;
        let target = (cur - need + p) % p;
        if let Some(&j) = mp.get(&target) {
            best = best.min(i as i32 - j);
        }
        mp.insert(cur, i as i32);
    }
    if best > nums.len() as i32 { -1 } else { best }
}
fn main() { println!("{}", min_subarray_len(vec![3,1,4,2], 6)); }
#[cfg(test)]
mod tests {
    use super::min_subarray_len;
    #[test]
    fn example_one() { assert_eq!(min_subarray_len(vec![3,1,4,2], 6), 1); }
    #[test]
    fn example_two() { assert_eq!(min_subarray_len(vec![6,3,5,2], 9), 2); }
}