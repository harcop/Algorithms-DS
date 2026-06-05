/// LeetCode #1769 - Minimum Number of Operations to Move All Balls to Each Box
fn min_operations(boxes: String) -> Vec<i32> {
    let b = boxes.as_bytes();
    let n = b.len();
    let mut ans = vec![0i32; n];
    let mut balls = 0i32;
    let mut ops = 0i32;
    for i in 1..n {
        if b[i - 1] == b'1' {
            balls += 1;
        }
        ops += balls;
        ans[i] = ops;
    }
    balls = 0;
    ops = 0;
    for i in (0..n - 1).rev() {
        if b[i + 1] == b'1' {
            balls += 1;
        }
        ops += balls;
        ans[i] += ops;
    }
    ans
}
fn main() { println!("{:?}", min_operations("110".into())); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations("110".into()), vec![1, 1, 3]); }
    #[test]
    fn example_two() { assert_eq!(min_operations("001011".into()), vec![11, 8, 5, 4, 3, 4]); }
}
