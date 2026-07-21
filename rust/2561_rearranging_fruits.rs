/// LeetCode #2561 - Rearranging Fruits
use std::collections::HashMap;

fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &a in &basket1 {
        *cnt.entry(a).or_insert(0) += 1;
    }
    for &b in &basket2 {
        *cnt.entry(b).or_insert(0) -= 1;
    }

    let mut mi = i32::MAX;
    let mut nums = Vec::new();
    for (&x, &v) in &cnt {
        if v % 2 != 0 {
            return -1;
        }
        for _ in 0..(v.abs() / 2) {
            nums.push(x);
        }
        mi = mi.min(x);
    }

    nums.sort_unstable();
    let m = nums.len();
    let mut ans = 0i64;
    for i in 0..(m / 2) {
        ans += (nums[i] as i64).min((mi as i64) * 2);
    }
    ans
}

fn main() {
    println!("{}", min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example_one() {
        assert_eq!(min_cost(vec![4, 2, 2, 2], vec![1, 4, 1, 2]), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_cost(vec![2, 3, 4, 1], vec![3, 2, 5, 1]), -1);
    }
}
