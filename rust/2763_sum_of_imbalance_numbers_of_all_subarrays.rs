/// LeetCode #2763 - Sum of Imbalance Numbers of All Subarrays
use std::collections::BTreeMap;

fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        let mut tree: BTreeMap<i32, usize> = BTreeMap::new();
        let mut cnt = 0;
        for j in i..n {
            let x = nums[j];
            let lo = tree.range(..x).next_back().map(|(&k, _)| k);
            let hi = tree.range(x..).next().map(|(&k, _)| k);
            if let Some(h) = hi {
                if h - x > 1 { cnt += 1; }
            }
            if let Some(l) = lo {
                if x - l > 1 { cnt += 1; }
            }
            if lo.is_some() && hi.is_some() && hi.unwrap() - lo.unwrap() > 1 {
                cnt -= 1;
            }
            *tree.entry(x).or_insert(0) += 1;
            ans += cnt;
        }
    }
    ans
}

fn main() {
    println!("{}", sum_imbalance_numbers(vec![2, 3, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::sum_imbalance_numbers;

    #[test]
    fn example_one() {
        assert_eq!(sum_imbalance_numbers(vec![2, 3, 1, 4]), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_imbalance_numbers(vec![1, 3, 3, 3, 5]), 8);
    }
}
