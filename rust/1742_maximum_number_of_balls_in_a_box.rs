/// LeetCode #1742 - Maximum Number of Balls in a Box
fn digit_sum(mut x: i32) -> i32 {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}
fn count_balls(low: i32, high: i32) -> i32 {
    let mut cnt = [0i32; 46];
    for b in low..=high {
        cnt[digit_sum(b) as usize] += 1;
    }
    *cnt.iter().max().unwrap()
}
fn main() { println!("{}", count_balls(19, 28)); }
#[cfg(test)]
mod tests {
    use super::count_balls;
    #[test]
    fn example_one() { assert_eq!(count_balls(19, 28), 2); }
    #[test]
    fn example_two() { assert_eq!(count_balls(1, 2), 1); }
}
