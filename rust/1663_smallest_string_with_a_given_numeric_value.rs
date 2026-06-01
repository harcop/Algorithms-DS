/// LeetCode #1663 - Smallest String With A Given Numeric Value
fn get_smallest_string(n: i32, k: i32) -> String {
    let mut rem = k - n;
    let mut ans = vec![b'a'; n as usize];
    for i in (0..n as usize).rev() {
        let add = rem.min(25);
        ans[i] += add as u8;
        rem -= add;
    }
    String::from_utf8(ans).unwrap()
}
fn main() { println!("{}", get_smallest_string(3, 27)); }
#[cfg(test)]
mod tests {
    use super::get_smallest_string;
    #[test]
    fn example_one() { assert_eq!(get_smallest_string(3, 27), "aay"); }
}