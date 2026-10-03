/// LeetCode #4049 - Count Values With Equally Spaced Occurrences II
use std::collections::HashMap;

fn count_special_integers(nums: Vec<i32>) -> i32 {
    let mut pos: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, x) in nums.into_iter().enumerate() {
        pos.entry(x).or_default().push(i);
    }
    let mut ans = 0;
    for p in pos.values() {
        if p.len() < 3 {
            continue;
        }
        let d = p[1] - p[0];
        if p.windows(2).all(|w| w[1] - w[0] == d) {
            ans += 1;
        }
    }
    ans
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
        assert_eq!(count_special_integers(vec![8, 8, 8, 8]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(count_special_integers(vec![8, 6, 6, 8, 8]), 0);
    }
}
