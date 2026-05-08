/// LeetCode #532 - K-diff Pairs in an Array
use std::collections::HashMap;

fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
    if k < 0 {
        return 0;
    }
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for x in nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    if k == 0 {
        return cnt.values().filter(|&&v| v > 1).count() as i32;
    }
    let mut ans = 0i32;
    for &x in cnt.keys() {
        if cnt.contains_key(&(x + k)) {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", find_pairs(vec![3, 1, 4, 1, 5], 2));
}

#[cfg(test)]
mod tests {
    use super::find_pairs;

    #[test]
    fn example_one() {
        assert_eq!(find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    }
}
