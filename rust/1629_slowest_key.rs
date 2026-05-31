/// LeetCode #1629 - Slowest Key
fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
    let b = keys_pressed.as_bytes();
    let mut best = 0i32;
    let mut ans = b[0];
    let mut prev = 0i32;
    for (i, &c) in b.iter().enumerate() {
        let dur = release_times[i] - prev;
        if dur > best || (dur == best && c > ans) {
            best = dur;
            ans = c;
        }
        prev = release_times[i];
    }
    ans as char
}
fn main() { println!("{}", slowest_key(vec![9,29,49,50], "cbcd".into())); }
#[cfg(test)]
mod tests {
    use super::slowest_key;
    #[test]
    fn example_one() { assert_eq!(slowest_key(vec![9,29,49,50], "cbcd".into()), 'c'); }
}