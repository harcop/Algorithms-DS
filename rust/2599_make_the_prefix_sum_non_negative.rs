/// LeetCode #2599 - Make the Prefix Sum Non-negative
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn make_pref_sum_non_negative(nums: Vec<i32>) -> i32 {
    let mut pq: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut ans = 0;
    let mut s = 0i64;
    for x in nums {
        s += x as i64;
        if x < 0 {
            pq.push(Reverse(x));
        }
        while s < 0 {
            if let Some(Reverse(v)) = pq.pop() {
                s -= v as i64;
                ans += 1;
            } else {
                break;
            }
        }
    }
    ans
}

fn main() {
    println!("{}", make_pref_sum_non_negative(vec![2, 3, -5, 4]));
}

#[cfg(test)]
mod tests {
    use super::make_pref_sum_non_negative;

    #[test]
    fn example_one() {
        assert_eq!(make_pref_sum_non_negative(vec![2, 3, -5, 4]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(make_pref_sum_non_negative(vec![3, -5, -2, 6]), 1);
    }
}
