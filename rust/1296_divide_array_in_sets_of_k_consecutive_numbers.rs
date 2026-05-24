/// LeetCode #1296 - Divide Array in Sets of K Consecutive Numbers
use std::collections::HashMap;

fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
    if k == 1 {
        return true;
    }
    if nums.len() as i32 % k != 0 {
        return false;
    }
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for x in nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut keys: Vec<i32> = cnt.keys().copied().collect();
    keys.sort_unstable();
    for start in keys {
        let c = *cnt.get(&start).unwrap_or(&0);
        if c == 0 {
            continue;
        }
        for i in 0..k {
            let key = start + i;
            let have = *cnt.get(&key).unwrap_or(&0);
            if have < c {
                return false;
            }
            *cnt.get_mut(&key).unwrap() -= c;
        }
    }
    true
}

fn main() {
    println!("{}", is_possible_divide(vec![1, 2, 3, 4, 5, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::is_possible_divide;

    #[test]
    fn example_one() {
        assert!(is_possible_divide(vec![1, 2, 3, 4, 5, 6], 3));
    }

    #[test]
    fn example_two() {
        assert!(!is_possible_divide(vec![1, 2, 3, 4, 4, 5, 6], 3));
    }
}
