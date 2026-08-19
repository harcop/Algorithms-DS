/// LeetCode #3289 - The Two Sneaky Numbers of Digitville
use std::collections::HashMap;

fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt = HashMap::new();
    for x in nums {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut ans: Vec<i32> = cnt
        .into_iter()
        .filter(|&(_, v)| v == 2)
        .map(|(x, _)| x)
        .collect();
    ans.sort_unstable();
    ans
}

fn main() {
    println!("{:?}", get_sneaky_numbers(vec![0, 1, 1, 0]));
}

#[cfg(test)]
mod tests {
    use super::get_sneaky_numbers;

    #[test]
    fn example1() {
        assert_eq!(get_sneaky_numbers(vec![0, 1, 1, 0]), vec![0, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]), vec![2, 3]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]),
            vec![4, 5]
        );
    }
}
