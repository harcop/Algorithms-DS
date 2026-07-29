/// LeetCode #2766 - Relocate Marbles
use std::collections::HashSet;

fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
    let mut pos: HashSet<i32> = nums.into_iter().collect();
    for (f, t) in move_from.into_iter().zip(move_to) {
        if f != t {
            pos.remove(&f);
            pos.insert(t);
        }
    }
    let mut ans: Vec<i32> = pos.into_iter().collect();
    ans.sort_unstable();
    ans
}

fn main() {
    println!(
        "{:?}",
        relocate_marbles(vec![1, 6, 7, 8], vec![1, 7, 2], vec![2, 9, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::relocate_marbles;

    #[test]
    fn example_one() {
        assert_eq!(
            relocate_marbles(vec![1, 6, 7, 8], vec![1, 7, 2], vec![2, 9, 5]),
            vec![5, 6, 8, 9]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            relocate_marbles(vec![1, 1, 3, 3], vec![1, 3], vec![2, 2]),
            vec![2]
        );
    }
}
