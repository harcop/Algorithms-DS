/// LeetCode #325 - Maximum Size Subarray Sum Equals k (prefix sums + HashMap)
use std::collections::HashMap;

fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
    let mut best = 0i32;
    let mut pref = 0i32;
    let mut first_index: HashMap<i32, i32> = HashMap::from([(0, -1)]);
    for (r, &x) in nums.iter().enumerate() {
        pref += x;
        if let Some(&li) = first_index.get(&(pref - k)) {
            best = best.max((r as i32) - li);
        }
        first_index.entry(pref).or_insert(r as i32);
    }
    best
}

fn main() {
    println!("{}", max_sub_array_len(vec![1, -1, 5, -2, 3], 3));
}

#[cfg(test)]
mod tests {
    use super::max_sub_array_len;

    #[test]
    fn examples() {
        assert_eq!(
            max_sub_array_len(vec![1, -1, 5, -2, 3], 3),
            4
        );
        assert_eq!(
            max_sub_array_len(vec![-2, -1, 2, 1], 1),
            2
        );
    }
}
