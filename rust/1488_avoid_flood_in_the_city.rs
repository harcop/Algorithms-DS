/// LeetCode #1488 - Avoid Flood In The City
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![1i32; rains.len()];
    let mut full: HashMap<i32, usize> = HashMap::new();
    let mut dry = BinaryHeap::new();
    for (i, &lake) in rains.iter().enumerate() {
        if lake > 0 {
            ans[i] = -1;
            if let Some(&prev) = full.get(&lake) {
                while dry.peek().map_or(true, |&Reverse(d)| d <= prev) { dry.pop(); }
                if let Some(Reverse(d)) = dry.pop() { ans[d] = lake; } else { return vec![]; }
            }
            full.insert(lake, i);
        }
        dry.push(Reverse(i));
    }
    ans
}
fn main() { println!("{:?}", avoid_flood(vec![1,2,0,0,2,1])); }
#[cfg(test)]
mod tests {
    use super::avoid_flood;
    #[test]
    fn example_one() { assert_eq!(avoid_flood(vec![1,2,0,0,2,1]), vec![-1,-1,2,1,-1,-1]); }
    #[test]
    fn example_two() { assert_eq!(avoid_flood(vec![1,2,3,4]), vec![-1,-1,-1,-1]); }
}