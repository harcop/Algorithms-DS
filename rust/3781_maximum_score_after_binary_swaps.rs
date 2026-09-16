/// LeetCode #3781 - Maximum Score After Binary Swaps
use std::collections::BinaryHeap;

fn maximum_score(nums: Vec<i32>, s: String) -> i64 {
    let mut ans = 0i64;
    let mut pq = BinaryHeap::new();
    for (x, c) in nums.into_iter().zip(s.bytes()) {
        pq.push(x);
        if c == b'1' {
            ans += pq.pop().unwrap() as i64;
        }
    }
    ans
}

fn main() {
    println!("{}", maximum_score(vec![2, 1, 5, 2, 3], "01010".into()));
}

#[cfg(test)]
mod tests {
    use super::maximum_score;

    #[test]
    fn example1() {
        assert_eq!(maximum_score(vec![2, 1, 5, 2, 3], "01010".into()), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_score(vec![4, 7, 2, 9], "0000".into()), 0);
    }
}
