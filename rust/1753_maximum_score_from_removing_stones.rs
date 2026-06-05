/// LeetCode #1753 - Maximum Score From Removing Stones
fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let mut v = [a, b, c];
    v.sort_unstable();
    if v[0] + v[1] <= v[2] {
        v[0] + v[1]
    } else {
        (a + b + c) / 2
    }
}
fn main() { println!("{}", maximum_score(2, 4, 6)); }
#[cfg(test)]
mod tests {
    use super::maximum_score;
    #[test]
    fn example_one() { assert_eq!(maximum_score(2, 4, 6), 6); }
    #[test]
    fn example_two() { assert_eq!(maximum_score(4, 4, 6), 7); }
}
