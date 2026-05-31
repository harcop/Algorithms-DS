/// LeetCode #1608 - Special Array With X Elements Greater Than Or Equal X
fn special_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    for x in 0..=n as i32 {
        let c = nums.iter().filter(|&&v| v >= x).count() as i32;
        if c == x { return x; }
    }
    -1
}
fn main() { println!("{}", special_array(vec![3,5])); }
#[cfg(test)]
mod tests {
    use super::special_array;
    #[test]
    fn example_one() { assert_eq!(special_array(vec![3,5]), 2); }
    #[test]
    fn example_two() { assert_eq!(special_array(vec![0,0]), -1); }
}