/// LeetCode #3731 - Find Missing Elements
use std::collections::HashSet;

fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
    let mn = *nums.iter().min().unwrap();
    let mx = *nums.iter().max().unwrap();
    let s: HashSet<i32> = nums.into_iter().collect();
    (mn + 1..mx).filter(|x| !s.contains(x)).collect()
}

fn main() {
    println!("{:?}", find_missing_elements(vec![1, 4, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::find_missing_elements;

    #[test]
    fn example1() {
        assert_eq!(find_missing_elements(vec![1, 4, 2, 5]), vec![3]);
    }

    #[test]
    fn example2() {
        assert_eq!(find_missing_elements(vec![7, 8, 6, 9]), Vec::<i32>::new());
    }

    #[test]
    fn example3() {
        assert_eq!(find_missing_elements(vec![5, 1]), vec![2, 3, 4]);
    }
}
