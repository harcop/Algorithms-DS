/// LeetCode #2780 - Minimum Index of a Valid Split
use std::collections::HashMap;

fn minimum_index(nums: Vec<i32>) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let (mut x, mut cnt) = (0, 0);
    for &v in &nums {
        let t = freq.entry(v).or_insert(0);
        *t += 1;
        if *t > cnt {
            cnt = *t;
            x = v;
        }
    }
    let mut cur = 0;
    for (i, &v) in nums.iter().enumerate() {
        let i = i + 1;
        if v == x {
            cur += 1;
            if cur * 2 > i as i32 && (cnt - cur) * 2 > (nums.len() - i) as i32 {
                return (i - 1) as i32;
            }
        }
    }
    -1
}

fn main() {
    println!("{}", minimum_index(vec![1, 2, 2, 2]));
}

#[cfg(test)]
mod tests {
    use super::minimum_index;

    #[test]
    fn example_one() {
        assert_eq!(minimum_index(vec![1, 2, 2, 2]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_index(vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1]), 4);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_index(vec![3, 3, 3, 3, 7, 2, 2]), -1);
    }
}
