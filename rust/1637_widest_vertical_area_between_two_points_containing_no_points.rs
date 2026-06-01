/// LeetCode #1637 - Widest Vertical Area Between Two Points Containing No Points
fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut xs: Vec<i32> = points.iter().map(|p| p[0]).collect();
    xs.sort_unstable();
    xs.dedup();
    let mut ans = 0i32;
    for w in xs.windows(2) { ans = ans.max(w[1] - w[0]); }
    ans
}
fn main() { println!("{}", max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]])); }
#[cfg(test)]
mod tests {
    use super::max_width_of_vertical_area;
    #[test]
    fn example_one() { assert_eq!(max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]]), 1); }
    #[test]
    fn example_two() { assert_eq!(max_width_of_vertical_area(vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]]), 3); }
}