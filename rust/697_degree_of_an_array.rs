/// LeetCode #697 - Degree of an Array
use std::collections::HashMap;

fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut first: HashMap<i32, usize> = HashMap::new();
    let mut last: HashMap<i32, usize> = HashMap::new();
    let mut count: HashMap<i32, i32> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        first.entry(x).or_insert(i);
        last.insert(x, i);
        *count.entry(x).or_insert(0) += 1;
    }
    let degree = *count.values().max().unwrap();
    let mut best = nums.len() as i32;
    for (k, &c) in &count {
        if c == degree {
            best = best.min((last[k] - first[k] + 1) as i32);
        }
    }
    best
}

fn main() {
    println!("{}", find_shortest_sub_array(vec![1,2,2,3,1]));
}

#[cfg(test)]
mod tests {
    use super::find_shortest_sub_array;

    #[test]
    fn example_one() {
        assert_eq!(find_shortest_sub_array(vec![1,2,2,3,1]), 2);
    }
}
