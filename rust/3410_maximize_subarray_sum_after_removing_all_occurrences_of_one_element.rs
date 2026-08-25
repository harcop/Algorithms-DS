/// LeetCode #3410 - Maximize Subarray Sum After Removing All Occurrences of One Element
fn max_subarray_sum(nums: Vec<i32>) -> i64 {
    use std::collections::HashMap;
    let mut result = i64::MIN;
    let mut curr = 0i64;
    let mut mn = 0i64;
    let mut mn0 = 0i64;
    let mut mn1: HashMap<i32, i64> = HashMap::new();
    for x in nums {
        curr += x as i64;
        result = result.max(curr - mn);
        let prev = *mn1.get(&x).unwrap_or(&0);
        let now = prev.min(mn0) + x as i64;
        mn1.insert(x, now);
        mn0 = mn0.min(curr);
        mn = mn.min(now).min(mn0);
    }
    result
}

fn main() {
    println!("{}", max_subarray_sum(vec![-3, 2, -2, -1, 3, -2, 3]));
}

#[cfg(test)]
mod tests {
    use super::max_subarray_sum;

    #[test]
    fn example1() {
        assert_eq!(max_subarray_sum(vec![-3, 2, -2, -1, 3, -2, 3]), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(max_subarray_sum(vec![1, 2, 3, 4]), 10);
    }
}
