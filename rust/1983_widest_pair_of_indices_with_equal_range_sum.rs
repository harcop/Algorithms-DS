/// LeetCode #1983 - Widest Pair of Indices With Equal Range Sum
use std::collections::HashMap;

fn widest_pair_of_indices(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut d: HashMap<i32, i32> = HashMap::from([(0, -1)]);
    let mut ans = 0i32;
    let mut s = 0i32;
    for (i, (a, b)) in nums1.iter().zip(nums2.iter()).enumerate() {
        s += a - b;
        if let Some(&j) = d.get(&s) {
            ans = ans.max(i as i32 - j);
        } else {
            d.insert(s, i as i32);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        widest_pair_of_indices(vec![1, 1, 0, 1], vec![0, 1, 1, 0])
    );
}

#[cfg(test)]
mod tests {
    use super::widest_pair_of_indices;

    #[test]
    fn example_one() {
        assert_eq!(
            widest_pair_of_indices(vec![1, 1, 0, 1], vec![0, 1, 1, 0]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(widest_pair_of_indices(vec![0, 1], vec![1, 1]), 1);
    }
}
