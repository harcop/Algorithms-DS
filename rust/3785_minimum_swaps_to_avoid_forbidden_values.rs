/// LeetCode #3785 - Minimum Swaps to Avoid Forbidden Values
use std::collections::HashMap;

fn min_swaps(nums: Vec<i32>, forbidden: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let mut cnt_nums: HashMap<i32, i32> = HashMap::new();
    let mut cnt_forb: HashMap<i32, i32> = HashMap::new();
    for &x in &nums {
        *cnt_nums.entry(x).or_insert(0) += 1;
    }
    for &x in &forbidden {
        *cnt_forb.entry(x).or_insert(0) += 1;
    }
    for (&k, &c) in &cnt_nums {
        if c + cnt_forb.get(&k).copied().unwrap_or(0) > n {
            return -1;
        }
    }
    let mut bad: HashMap<i32, i32> = HashMap::new();
    let mut tot = 0;
    let mut mx = 0;
    for i in 0..nums.len() {
        if nums[i] == forbidden[i] {
            tot += 1;
            let e = bad.entry(nums[i]).or_insert(0);
            *e += 1;
            mx = mx.max(*e);
        }
    }
    mx.max((tot + 1) / 2)
}

fn main() {
    println!("{}", min_swaps(vec![1, 2, 3], vec![3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_swaps;

    #[test]
    fn example1() {
        assert_eq!(min_swaps(vec![1, 2, 3], vec![3, 2, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_swaps(vec![4, 6, 6, 5], vec![4, 6, 5, 5]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(min_swaps(vec![7, 7], vec![8, 7]), -1);
    }

    #[test]
    fn example4() {
        assert_eq!(min_swaps(vec![1, 2], vec![2, 1]), 0);
    }
}
