/// LeetCode #1187 - Make Array Strictly Increasing
use std::collections::HashMap;

fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    let mut b = arr2;
    b.sort_unstable();
    b.dedup();
    let mut memo: HashMap<(usize, i32), i32> = HashMap::new();
    fn dfs(
        i: usize,
        prev: i32,
        arr1: &[i32],
        b: &[i32],
        memo: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if i == arr1.len() {
            return 0;
        }
        if let Some(&v) = memo.get(&(i, prev)) {
            return v;
        }
        let mut res = i32::MAX / 2;
        if arr1[i] > prev {
            res = dfs(i + 1, arr1[i], arr1, b, memo);
        }
        let idx = b.partition_point(|&v| v <= prev);
        if idx < b.len() {
            res = res.min(1 + dfs(i + 1, b[idx], arr1, b, memo));
        }
        memo.insert((i, prev), res);
        res
    }
    let ans = dfs(0, i32::MIN, &arr1, &b, &mut memo);
    if ans >= i32::MAX / 2 {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::make_array_increasing;

    #[test]
    fn example_one() {
        assert_eq!(make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]), -1);
    }
}
