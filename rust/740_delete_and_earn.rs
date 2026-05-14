/// LeetCode #740 - Delete and Earn
use std::cmp::max;
use std::collections::HashMap;

fn delete_and_earn(nums: Vec<i32>) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::new();
    let mut mx = 0i32;
    for x in nums {
        *freq.entry(x).or_insert(0) += x;
        mx = mx.max(x);
    }
    let mut take = 0i32;
    let mut skip = 0i32;
    let mut prev = -1i32;
    let mut keys: Vec<i32> = freq.keys().copied().collect();
    keys.sort();
    for k in keys {
        let earn = *freq.get(&k).unwrap();
        if k != prev + 1 {
            let best = max(take, skip);
            take = best + earn;
            skip = best;
        } else {
            let new_take = skip + earn;
            let new_skip = max(take, skip);
            take = new_take;
            skip = new_skip;
        }
        prev = k;
    }
    max(take, skip)
}

fn main() {
    println!("{}", delete_and_earn(vec![3, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::delete_and_earn;

    #[test]
    fn example_one() {
        assert_eq!(delete_and_earn(vec![3, 4, 2]), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }
}
