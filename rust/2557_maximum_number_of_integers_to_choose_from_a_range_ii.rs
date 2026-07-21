/// LeetCode #2557 - Maximum Number of Integers to Choose From a Range II
use std::collections::HashSet;

fn max_count(banned: Vec<i32>, n: i32, mut max_sum: i64) -> i32 {
    let mut ban: Vec<i32> = banned
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .filter(|&x| x <= n)
        .collect();
    ban.push(0);
    ban.push(n + 1);
    ban.sort_unstable();
    ban.dedup();

    let mut ans = 0i32;
    for w in ban.windows(2) {
        let i = w[0];
        let j = w[1];
        let mut left = 0i32;
        let mut right = j - i - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            let sum = (i as i64 + 1 + i as i64 + mid as i64) * mid as i64 / 2;
            if sum <= max_sum {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        ans += left;
        max_sum -= (i as i64 + 1 + i as i64 + left as i64) * left as i64 / 2;
        if max_sum <= 0 {
            break;
        }
    }
    ans
}

fn main() {
    println!("{}", max_count(vec![1, 4, 6], 6, 4));
}

#[cfg(test)]
mod tests {
    use super::max_count;

    #[test]
    fn example_one() {
        assert_eq!(max_count(vec![1, 4, 6], 6, 4), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_count(vec![4, 3, 5, 6], 7, 18), 3);
    }
}
