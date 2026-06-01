/// LeetCode #1698 - Number Of Distinct Substrings In A String
fn count_distinct(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        let mut h = 0u64;
        for j in i..n {
            h = h.wrapping_mul(131).wrapping_add(b[j] as u64);
            set.insert(h);
        }
    }
    set.len() as i32
}
fn main() { println!("{}", count_distinct("aabbaba".into())); }
#[cfg(test)]
mod tests {
    use super::count_distinct;
    #[test]
    fn example_one() { assert_eq!(count_distinct("aabbaba".into()), 21); }
}