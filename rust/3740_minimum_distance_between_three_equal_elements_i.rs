/// LeetCode #3740 - Minimum Distance Between Three Equal Elements I
use std::collections::HashMap;

fn minimum_distance(nums: Vec<i32>) -> i32 {
    let mut g: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        g.entry(x).or_default().push(i);
    }
    let mut ans = i32::MAX;
    for ls in g.values() {
        if ls.len() < 3 {
            continue;
        }
        for h in 0..ls.len() - 2 {
            ans = ans.min(((ls[h + 2] - ls[h]) as i32) * 2);
        }
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", minimum_distance(vec![1, 2, 1, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_distance;

    #[test]
    fn example1() {
        assert_eq!(minimum_distance(vec![1, 2, 1, 1, 3]), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_distance(vec![1]), -1);
    }
}
