/// LeetCode #1465 - Maximum Area Of A Piece Of Cake After Horizontal And Vertical Cuts
fn max_area(h: i32, w: i32, mut horizontal: Vec<i32>, mut vertical: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    horizontal.sort_unstable();
    vertical.sort_unstable();
    let mut max_h = (horizontal.get(0).copied().unwrap_or(0) as i64).max(h as i64 - horizontal.last().copied().unwrap_or(0) as i64);
    for i in 1..horizontal.len() {
        max_h = max_h.max((horizontal[i] - horizontal[i - 1]) as i64);
    }
    let mut max_v = (vertical.get(0).copied().unwrap_or(0) as i64).max(w as i64 - vertical.last().copied().unwrap_or(0) as i64);
    for i in 1..vertical.len() {
        max_v = max_v.max((vertical[i] - vertical[i - 1]) as i64);
    }
    ((max_h * max_v) % MOD) as i32
}
fn main() { println!("{}", max_area(5, 4, vec![1,2,4], vec![1,3])); }
#[cfg(test)]
mod tests {
    use super::max_area;
    #[test]
    fn example_one() { assert_eq!(max_area(5, 4, vec![1,2,4], vec![1,3]), 4); }
    #[test]
    fn example_two() { assert_eq!(max_area(5, 4, vec![3,1], vec![1]), 6); }
}