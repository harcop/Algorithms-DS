/// LeetCode #1566 - Detect Pattern Of Length M Repeated K Or More Times
fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
    let m = m as usize;
    let k = k as usize;
    if m == 0 || k < 2 || m * k > arr.len() { return false; }
    for i in 0..=arr.len() - m * k {
        let pat = &arr[i..i + m];
        let mut ok = true;
        for j in 1..k {
            if arr[i + j * m..i + (j + 1) * m] != *pat {
                ok = false;
                break;
            }
        }
        if ok { return true; }
    }
    false
}
fn main() { println!("{}", contains_pattern(vec![1,2,4,4,4,4], 1, 3)); }
#[cfg(test)]
mod tests {
    use super::contains_pattern;
    #[test]
    fn example_one() { assert!(contains_pattern(vec![1,2,4,4,4,4], 1, 3)); }
    #[test]
    fn example_two() { assert!(contains_pattern(vec![1,2,1,2,1,2,1,2,1,2], 2, 2)); }
}