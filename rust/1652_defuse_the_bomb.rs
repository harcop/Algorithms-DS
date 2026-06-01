/// LeetCode #1652 - Defuse The Bomb
fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
    let n = code.len();
    if k == 0 { return vec![0; n]; }
    let kabs = k.unsigned_abs() as usize;
    let mut ans = vec![0; n];
    for i in 0..n {
        let mut s = 0i32;
        for j in 1..=kabs {
            let idx = if k > 0 { (i + j) % n } else { (i + n - j) % n };
            s += code[idx];
        }
        ans[i] = s;
    }
    ans
}
fn main() { println!("{:?}", decrypt(vec![5,7,1,4], 3)); }
#[cfg(test)]
mod tests {
    use super::decrypt;
    #[test]
    fn example_one() { assert_eq!(decrypt(vec![5,7,1,4], 3), vec![12,10,16,13]); }
    #[test]
    fn example_two() { assert_eq!(decrypt(vec![1,2,3,4], 0), vec![0,0,0,0]); }
}