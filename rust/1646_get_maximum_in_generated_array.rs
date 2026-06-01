/// LeetCode #1646 - Get Maximum In Generated Array
fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 { return 0; }
    let n = n as usize;
    let mut a = vec![0i32; n + 1];
    a[1] = 1;
    for i in 2..=n {
        a[i] = if i % 2 == 0 { a[i / 2] } else { a[i / 2] + a[i / 2 + 1] };
    }
    *a.iter().max().unwrap()
}
fn main() { println!("{}", get_maximum_generated(7)); }
#[cfg(test)]
mod tests {
    use super::get_maximum_generated;
    #[test]
    fn example_one() { assert_eq!(get_maximum_generated(7), 3); }
    #[test]
    fn example_two() { assert_eq!(get_maximum_generated(2), 1); }
}