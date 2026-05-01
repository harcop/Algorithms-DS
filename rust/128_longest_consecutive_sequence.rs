use std::collections::HashSet;

/// LeetCode #128 - Longest Consecutive Sequence
fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut best = 0;
    for &n in &set {
        if !set.contains(&(n - 1)) {
            let mut len = 1;
            let mut x = n + 1;
            while set.contains(&x) {
                len += 1;
                x += 1;
            }
            best = best.max(len);
        }
    }
    best
}

fn main() {
    println!("{}", longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::longest_consecutive;

    #[test]
    fn example_one() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
