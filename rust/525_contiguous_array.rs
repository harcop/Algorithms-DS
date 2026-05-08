/// LeetCode #525 - Contiguous Array
use std::collections::HashMap;

fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut first: HashMap<i32, i32> = HashMap::new();
    first.insert(0, -1);
    let mut bal = 0i32;
    let mut best = 0i32;
    for (i, x) in nums.into_iter().enumerate() {
        bal += if x == 1 { 1 } else { -1 };
        let i = i as i32;
        if let Some(&j) = first.get(&bal) {
            best = best.max(i - j);
        } else {
            first.insert(bal, i);
        }
    }
    best
}

fn main() {
    println!("{}", find_max_length(vec![0, 1]));
}

#[cfg(test)]
mod tests {
    use super::find_max_length;

    #[test]
    fn example_one() {
        assert_eq!(find_max_length(vec![0, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_max_length(vec![0, 1, 0]), 2);
    }
}
