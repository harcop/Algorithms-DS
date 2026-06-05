/// LeetCode #1758 - Minimum Changes To Make Alternating Binary String
fn min_operations(s: String) -> i32 {
    let b = s.as_bytes();
    let mut c0 = 0i32;
    let mut c1 = 0i32;
    for (i, &ch) in b.iter().enumerate() {
        if ch == b'0' {
            if i % 2 == 0 {
                c0 += 1;
            } else {
                c1 += 1;
            }
        } else if i % 2 == 0 {
            c1 += 1;
        } else {
            c0 += 1;
        }
    }
    c0.min(c1)
}
fn main() { println!("{}", min_operations("0011".into())); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations("0011".into()), 2); }
    #[test]
    fn example_two() { assert_eq!(min_operations("010101".into()), 0); }
}
