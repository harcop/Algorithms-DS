/// LeetCode #1732 - Find the Highest Altitude
fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut h = 0;
    for v in gain {
        h += v;
        ans = ans.max(h);
    }
    ans
}
fn main() { println!("{}", largest_altitude(vec![-5, 1, 5, 0, -7])); }
#[cfg(test)]
mod tests {
    use super::largest_altitude;
    #[test]
    fn example_one() {
        assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    }
    #[test]
    fn example_two() {
        assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
