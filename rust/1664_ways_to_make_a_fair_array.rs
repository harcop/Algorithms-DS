/// LeetCode #1664 - Ways To Make A Fair Array
fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut o = 0i64;
    let mut e = 0i64;
    for (i, &x) in nums.iter().enumerate() {
        if i % 2 == 0 { e += x as i64; } else { o += x as i64; }
    }
    let mut po = 0i64;
    let mut pe = 0i64;
    let mut ans = 0i32;
    for i in 0..n {
        let so = o - po - if i % 2 == 1 { nums[i] as i64 } else { 0 };
        let se = e - pe - if i % 2 == 0 { nums[i] as i64 } else { 0 };
        if i % 2 == 0 {
            if pe + so == po + se { ans += 1; }
        } else if po + se == pe + so {
            ans += 1;
        }
        if i % 2 == 0 { pe += nums[i] as i64; } else { po += nums[i] as i64; }
    }
    ans
}
fn main() { println!("{}", ways_to_make_fair(vec![2,1,6,4])); }
#[cfg(test)]
mod tests {
    use super::ways_to_make_fair;
    #[test]
    fn example_one() { assert_eq!(ways_to_make_fair(vec![2,1,6,4]), 1); }
}