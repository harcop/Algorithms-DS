/// LeetCode #1737 - Change Minimum Characters to Satisfy One of Three Conditions
fn min_characters(a: String, b: String) -> i32 {
    let m = a.len();
    let n = b.len();
    let mut cnt1 = [0i32; 26];
    let mut cnt2 = [0i32; 26];
    for c in a.bytes() {
        cnt1[(c - b'a') as usize] += 1;
    }
    for c in b.bytes() {
        cnt2[(c - b'a') as usize] += 1;
    }
    let mut ans = (m + n) as i32;
    for i in 0..26 {
        ans = ans.min((m + n - cnt1[i] as usize - cnt2[i] as usize) as i32);
    }
    let f = |cnt1: &[i32; 26], cnt2: &[i32; 26], ans: &mut i32| {
        for i in 1..26 {
            let mut t = 0;
            for j in i..26 {
                t += cnt1[j];
            }
            for j in 0..i {
                t += cnt2[j];
            }
            *ans = (*ans).min(t);
        }
    };
    f(&cnt1, &cnt2, &mut ans);
    f(&cnt2, &cnt1, &mut ans);
    ans
}
fn main() { println!("{}", min_characters("aba".into(), "caa".into())); }
#[cfg(test)]
mod tests {
    use super::min_characters;
    #[test]
    fn example_one() { assert_eq!(min_characters("aba".into(), "caa".into()), 2); }
    #[test]
    fn example_two() { assert_eq!(min_characters("dabadd".into(), "cda".into()), 3); }
}
