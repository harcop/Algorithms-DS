/// LeetCode #3011 - Find if Array Can Be Sorted
use std::collections::HashMap;

fn popcount(x: i32) -> u32 {
    x.count_ones()
}

fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut groups: HashMap<u32, Vec<i32>> = HashMap::new();
    for &x in &nums {
        groups.entry(popcount(x)).or_default().push(x);
    }
    for v in groups.values_mut() {
        v.sort_unstable();
    }

    let mut ptr: HashMap<u32, usize> = HashMap::new();
    let mut rebuilt = Vec::with_capacity(nums.len());
    for &x in &nums {
        let pc = popcount(x);
        let i = *ptr.entry(pc).or_insert(0);
        rebuilt.push(groups[&pc][i]);
        ptr.insert(pc, i + 1);
    }

    rebuilt.windows(2).all(|w| w[0] <= w[1])
}

fn main() {
    println!("{}", can_sort_array(vec![8, 4, 2, 30, 15]));
    println!("{}", can_sort_array(vec![1, 2, 3, 4, 5]));
    println!("{}", can_sort_array(vec![3, 16, 8, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::can_sort_array;

    #[test]
    fn example_one() {
        assert!(can_sort_array(vec![8, 4, 2, 30, 15]));
    }

    #[test]
    fn example_two() {
        assert!(can_sort_array(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn example_three() {
        assert!(!can_sort_array(vec![3, 16, 8, 4, 2]));
    }
}
