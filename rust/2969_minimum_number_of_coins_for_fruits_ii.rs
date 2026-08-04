/// LeetCode #2969 - Minimum Number of Coins for Fruits II
use std::collections::VecDeque;

fn minimum_coins(mut prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in (1..=n).rev() {
        while let Some(&front) = q.front() {
            if front > i * 2 + 1 {
                q.pop_front();
            } else {
                break;
            }
        }
        if i <= (n - 1) / 2 {
            prices[i - 1] += prices[*q.front().unwrap() - 1];
        }
        while let Some(&back) = q.back() {
            if prices[back - 1] >= prices[i - 1] {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back(i);
    }
    prices[0]
}

fn main() {
    println!("{}", minimum_coins(vec![3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimum_coins;

    #[test]
    fn example_one() {
        assert_eq!(minimum_coins(vec![3, 1, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_coins(vec![1, 10, 1, 1]), 2);
    }
}
