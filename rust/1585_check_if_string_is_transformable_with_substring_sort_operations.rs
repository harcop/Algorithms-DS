/// LeetCode #1585 - Check If String Is Transformable With Substring Sort Operations
fn is_transformable(s: String, t: String) -> bool {
    let mut cnt = [0i32; 26];
    for c in s.bytes() { cnt[(c - b'a') as usize] += 1; }
    for c in t.bytes() {
        let i = (c - b'a') as usize;
        cnt[i] -= 1;
        if cnt[i] < 0 { return false; }
    }
    let mut pos = vec![std::collections::VecDeque::new(); 26];
    for (i, c) in s.bytes().enumerate() {
        pos[(c - b'a') as usize].push_back(i);
    }
    for c in t.bytes() {
        let i = (c - b'a') as usize;
        let p = pos[i].pop_front().unwrap();
        for j in 0..i {
            if let Some(&front) = pos[j].front() {
                if front < p { return false; }
            }
        }
    }
    true
}
fn main() { println!("{}", is_transformable("cba".into(), "abc".into())); }
#[cfg(test)]
mod tests {
    use super::is_transformable;
    #[test]
    fn example_one() { assert!(is_transformable("cba".into(), "abc".into())); }
    #[test]
    fn example_two() { assert!(!is_transformable("leetcode".into(), "codeleet".into())); }
}