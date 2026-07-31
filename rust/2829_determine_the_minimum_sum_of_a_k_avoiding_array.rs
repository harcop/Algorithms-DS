/// LeetCode #2829 - Determine the Minimum Sum of a k-Avoiding Array
use std::collections::HashSet;

fn minimum_sum(n: i32, k: i32) -> i32 {
    let mut s = 0;
    let mut i = 1;
    let mut vis = HashSet::new();
    for _ in 0..n {
        while vis.contains(&i) {
            i += 1;
        }
        vis.insert(k - i);
        s += i;
        i += 1;
    }
    s
}

fn main() {
    println!("{}", minimum_sum(5, 4));
}

#[cfg(test)]
mod tests {
    use super::minimum_sum;

    #[test]
    fn example_one() {
        assert_eq!(minimum_sum(5, 4), 18);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_sum(2, 6), 3);
    }
}
