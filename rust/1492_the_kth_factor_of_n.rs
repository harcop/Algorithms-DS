/// LeetCode #1492 - The Kth Factor Of N
fn kth_factor(n: i32, k: i32) -> i32 {
    let mut small = Vec::new();
    let mut large = Vec::new();
    let mut i = 1i32;
    while (i as i64) * (i as i64) <= n as i64 {
        if n % i == 0 {
            small.push(i);
            if i * i != n { large.push(n / i); }
        }
        i += 1;
    }
    large.reverse();
    small.extend(large);
    if k as usize <= small.len() { small[k as usize - 1] } else { -1 }
}
fn main() { println!("{}", kth_factor(12, 3)); }
#[cfg(test)]
mod tests {
    use super::kth_factor;
    #[test]
    fn example_one() { assert_eq!(kth_factor(12, 3), 3); }
    #[test]
    fn example_two() { assert_eq!(kth_factor(7, 2), 7); }
    #[test]
    fn example_three() { assert_eq!(kth_factor(4, 4), -1); }
}