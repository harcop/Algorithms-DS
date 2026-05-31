/// LeetCode #1625 - Lexicographically Smallest String After Applying Operations
fn smallest_string(s: String, k: i32) -> String {
    let mut a: Vec<u8> = s.into_bytes();
    let mut k = k;
    for i in 0..a.len() {
        if k <= 0 { break; }
        let cur = a[i];
        let mut best = cur;
        let start = if i == 0 { b'b' } else { b'a' };
        for c in start..=b'z' {
            let cost = if i == 0 && c == b'a' {
                (cur as i32 - b'z' as i32).abs()
            } else {
                (cur as i32 - c as i32).abs()
            };
            if cost <= k && c < best { best = c; }
        }
        let cost = if i == 0 && best == b'a' {
            (cur as i32 - b'z' as i32).abs()
        } else {
            (cur as i32 - best as i32).abs()
        };
        k -= cost;
        a[i] = best;
    }
    String::from_utf8(a).unwrap()
}
fn main() { println!("{}", smallest_string("cbabc".into(), 2)); }
#[cfg(test)]
mod tests {
    use super::smallest_string;
    #[test]
    fn example_one() { assert_eq!(smallest_string("cbabc".into(), 2), "baabc"); }
}