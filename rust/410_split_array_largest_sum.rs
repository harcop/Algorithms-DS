/// LeetCode #410 - Split Array Largest Sum (binary search answer)
fn split_array(nums: Vec<i32>, m: i32) -> i32 {
    let mut lo = *nums.iter().max().unwrap();
    let mut hi: i32 = nums.iter().sum();
    let m = m as usize;
    fn ok(nums: &[i32], cap: i32, m: usize) -> bool {
        let mut parts = 1usize;
        let mut cur = 0i64;
        for &x in nums {
            let x = x as i64;
            if x > cap as i64 {
                return false;
            }
            if cur + x > cap as i64 {
                parts += 1;
                cur = x;
            } else {
                cur += x;
            }
        }
        parts <= m
    }
    while lo < hi {
        let mid = (lo + hi) / 2;
        if ok(&nums, mid, m) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

fn main(){ println!("{}", split_array(vec![7,2,5,10,8], 2)); }

#[cfg(test)] mod tests { use super::*; #[test] fn ex(){
    assert_eq!(split_array(vec![7,2,5,10,8], 2), 18);
}}
