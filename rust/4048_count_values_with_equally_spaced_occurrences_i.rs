/// LeetCode #4048 - Count Values With Equally Spaced Occurrences I
use std::collections::HashMap;

fn count_special_integers(nums: Vec<i32>) -> i32 {
    let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, x) in nums.into_iter().enumerate() {
        pos.entry(x).or_default().push(i);
    }
    pos.values()
        .filter(|p| p.len() == 3 && p[0] + p[2] == p[1] * 2)
        .count() as i32
}

fn main() {
    println!("{}", count_special_integers(vec![1, 8, 1, 5, 1, 5, 8, 5]));
}

#[cfg(test)]
mod tests {
    use super::count_special_integers;

    #[test]
    fn example1() {
        assert_eq!(count_special_integers(vec![1, 8, 1, 5, 1, 5, 8, 5]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_special_integers(vec![8, 8, 8, 8]), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(count_special_integers(vec![8, 6, 6, 8, 8]), 0);
    }
}
