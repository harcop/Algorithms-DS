/// LeetCode #1481 - Least Number Of Unique Integers After K Removals
use std::collections::HashMap;
fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut freq = HashMap::new();
    for x in arr { *freq.entry(x).or_insert(0) += 1; }
    let mut counts: Vec<i32> = freq.values().copied().collect();
    counts.sort_unstable();
    let mut rem = k;
    let mut uniq = counts.len() as i32;
    for c in counts {
        if rem >= c { rem -= c; uniq -= 1; } else { break; }
    }
    uniq
}
fn main() { println!("{}", find_least_num_of_unique_ints(vec![5,5,4], 1)); }
#[cfg(test)]
mod tests {
    use super::find_least_num_of_unique_ints;
    #[test]
    fn example_one() { assert_eq!(find_least_num_of_unique_ints(vec![5,5,4], 1), 1); }
    #[test]
    fn example_two() { assert_eq!(find_least_num_of_unique_ints(vec![4,3,1,1,3,3,2], 3), 2); }
}