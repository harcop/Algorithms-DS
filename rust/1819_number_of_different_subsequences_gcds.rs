/// LeetCode #1819 - Number of Different Subsequences GCDs
use std::collections::HashSet;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn count_different_subsequence_gcds(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap();
    let vis: HashSet<i32> = nums.into_iter().collect();
    let mut ans = 0i32;
    for x in 1..=mx {
        let mut g = 0i32;
        let mut y = x;
        while y <= mx {
            if vis.contains(&y) {
                g = gcd(g, y);
                if g == x {
                    ans += 1;
                    break;
                }
            }
            y += x;
        }
    }
    ans
}

fn main() {
    println!("{}", count_different_subsequence_gcds(vec![6, 10, 3]));
}

#[cfg(test)]
mod tests {
    use super::count_different_subsequence_gcds;

    #[test]
    fn example_one() {
        assert_eq!(count_different_subsequence_gcds(vec![6, 10, 3]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_different_subsequence_gcds(vec![5, 15, 40, 5, 6]), 7);
    }
}
