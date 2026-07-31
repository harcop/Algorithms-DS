/// LeetCode #2808 - Minimum Seconds to Equalize a Circular Array
use std::collections::HashMap;

fn minimum_seconds(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut d: HashMap<i32, Vec<usize>> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        d.entry(x).or_default().push(i);
    }
    let mut ans = i32::MAX;
    for idx in d.values() {
        let m = idx.len();
        let mut t = idx[0] + n - idx[m - 1];
        for i in 1..m {
            t = t.max(idx[i] - idx[i - 1]);
        }
        ans = ans.min((t / 2) as i32);
    }
    ans
}

fn main() {
    println!("{}", minimum_seconds(vec![1, 2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimum_seconds;

    #[test]
    fn example_one() {
        assert_eq!(minimum_seconds(vec![1, 2, 1, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_seconds(vec![2, 1, 3, 3, 2]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_seconds(vec![5, 5, 5, 5]), 0);
    }
}
