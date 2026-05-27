/// LeetCode #1441 - Build An Array With Stack Operations
fn build_array(target: Vec<i32>, _n: i32) -> Vec<String> {
    let mut res = Vec::new();
    let mut cur = 1;
    for &t in &target {
        while cur < t {
            res.push("Push".into());
            res.push("Pop".into());
            cur += 1;
        }
        res.push("Push".into());
        cur += 1;
    }
    res
}
fn main() { println!("{:?}", build_array(vec![1, 3], 3)); }
#[cfg(test)]
mod tests {
    use super::build_array;
    #[test]
    fn example_one() {
        assert_eq!(build_array(vec![1, 3], 3), vec!["Push".to_string(),"Push".to_string(),"Pop".to_string(),"Push".to_string()]);
    }
    #[test]
    fn example_two() {
        assert_eq!(build_array(vec![1, 2, 3], 3), vec!["Push".to_string(),"Push".to_string(),"Push".to_string()]);
    }
}