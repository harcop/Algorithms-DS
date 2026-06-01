/// LeetCode #1640 - Check Array Formation Through Concatenation
use std::collections::HashMap;

fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    let mut pos = HashMap::new();
    for (i, &x) in arr.iter().enumerate() { pos.insert(x, i); }
    for p in &pieces {
        let start = match pos.get(&p[0]) { Some(&s) => s, None => return false };
        if start + p.len() > arr.len() { return false; }
        if arr[start..start + p.len()] != p[..] { return false; }
    }
    true
}
fn main() { println!("{}", can_form_array(vec![15,88], vec![vec![88]])); }
#[cfg(test)]
mod tests {
    use super::can_form_array;
    #[test]
    fn example_one() { assert!(can_form_array(vec![15,88], vec![vec![88]])); }
    #[test]
    fn example_two() { assert!(!can_form_array(vec![49,18,16], vec![vec![16,18,49]])); }
}