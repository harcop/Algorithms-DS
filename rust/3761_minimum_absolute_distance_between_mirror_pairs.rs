/// LeetCode #3761 - Minimum Absolute Distance Between Mirror Pairs
use std::collections::HashMap;

fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
    fn reverse(mut x: i32) -> i32 {
        let mut y = 0;
        while x > 0 {
            y = y * 10 + x % 10;
            x /= 10;
        }
        y
    }
    let n = nums.len() as i32;
    let mut ans = n + 1;
    let mut pos: HashMap<i32, i32> = HashMap::new();
    for (i, &v) in nums.iter().enumerate() {
        if let Some(&j) = pos.get(&v) {
            ans = ans.min(i as i32 - j);
        }
        pos.insert(reverse(v), i as i32);
    }
    if ans > n {
        -1
    } else {
        ans
    }
}

fn main() {
    println!(
        "{}",
        min_mirror_pair_distance(vec![12, 21, 45, 33, 54])
    );
}

#[cfg(test)]
mod tests {
    use super::min_mirror_pair_distance;

    #[test]
    fn example1() {
        assert_eq!(min_mirror_pair_distance(vec![12, 21, 45, 33, 54]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_mirror_pair_distance(vec![120, 21]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(min_mirror_pair_distance(vec![21, 120]), -1);
    }
}
