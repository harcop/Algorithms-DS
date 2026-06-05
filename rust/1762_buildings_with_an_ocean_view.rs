/// LeetCode #1762 - Buildings With an Ocean View
fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let mut max_h = 0i32;
    for (i, &h) in heights.iter().enumerate().rev() {
        if h > max_h {
            ans.push(i as i32);
            max_h = h;
        }
    }
    ans.reverse();
    ans
}
fn main() { println!("{:?}", find_buildings(vec![4, 3, 2, 10])); }
#[cfg(test)]
mod tests {
    use super::find_buildings;
    #[test]
    fn example_one() { assert_eq!(find_buildings(vec![4, 3, 2, 10]), vec![3]); }
    #[test]
    fn example_two() { assert_eq!(find_buildings(vec![4, 3, 2, 10, 1]), vec![3, 4]); }
}
