/// LeetCode #822 - Card Flipping Game
use std::collections::HashSet;

fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
    let mut bad = HashSet::new();
    for i in 0..fronts.len() {
        if fronts[i] == backs[i] {
            bad.insert(fronts[i]);
        }
    }
    let mut ans = i32::MAX;
    for i in 0..fronts.len() {
        for &x in &[fronts[i], backs[i]] {
            if !bad.contains(&x) {
                ans = ans.min(x);
            }
        }
    }
    if ans == i32::MAX {
        0
    } else {
        ans
    }
}

fn main() {
    println!("{}", flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 1, 3, 7]));
}

#[cfg(test)]
mod tests {
    use super::flipgame;

    #[test]
    fn example_one() {
        assert_eq!(flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 1, 3, 7]), 2);
    }
}
