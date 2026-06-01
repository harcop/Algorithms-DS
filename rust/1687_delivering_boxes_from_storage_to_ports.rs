/// LeetCode #1687 - Delivering Boxes From Storage To Ports
fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
    let n = boxes.len();
    let mut dp = vec![i32::MAX; n + 1];
    dp[0] = 0;
    let mut w = 0i64;
    let mut cnt = 0i32;
    let mut l = 0usize;
    for r in 0..n {
        w += boxes[r][2] as i64;
        cnt += 1;
        if r > 0 && boxes[r][0] != boxes[r - 1][1] { /* port change counted in trip formula */ }
        while l <= r && (cnt > max_boxes || w > max_weight as i64) {
            w -= boxes[l][2] as i64;
            cnt -= 1;
            l += 1;
        }
        let mut cost = dp[l] + (r - l + 1) as i32;
        if l > 0 { cost += 1; }
        if boxes[l][0] != boxes[r][1] { cost += 1; }
        dp[r + 1] = dp[r + 1].min(cost);
        for i in l..r {
            if boxes[i][1] != boxes[i + 1][0] {
                let c = dp[i + 1] + (r - i) as i32 + if i + 1 > 0 { 1 } else { 0 } + 1;
                dp[r + 1] = dp[r + 1].min(c);
            }
        }
    }
    dp[n]
}
fn main() { println!("{}", box_delivering(vec![vec![2,3,3],vec![3,3,3],vec![3,3,3],vec![2,3,3]], 3, 3, 10)); }
#[cfg(test)]
mod tests {
    use super::box_delivering;
    #[test]
    fn example_one() { assert_eq!(box_delivering(vec![vec![2,3,3],vec![3,3,3],vec![3,3,3],vec![2,3,3]], 3, 3, 10), 6); }
}