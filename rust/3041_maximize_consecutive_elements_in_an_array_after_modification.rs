/// LeetCode #3041 - Maximize Consecutive Elements in an Array After Modification
use std::collections::HashMap;

fn max_consecutive(nums: Vec<i32>) -> i32 {
    let mut sorted = nums;
    sorted.sort_unstable();
    let mut dp: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;
    for num in sorted {
        let from_prev = *dp.get(&(num - 1)).unwrap_or(&0);
        let from_self = *dp.get(&num).unwrap_or(&0);
        dp.insert(num + 1, from_self + 1);
        dp.insert(num, from_prev + 1);
        ans = ans.max(*dp.get(&num).unwrap()).max(*dp.get(&(num + 1)).unwrap());
    }
    ans
}

fn main() {
    println!("{}", max_consecutive(vec![2, 1, 5, 1, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_consecutive;

    #[test]
    fn example1() {
        assert_eq!(max_consecutive(vec![2, 1, 5, 1, 1]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_consecutive(vec![1, 4, 7, 10]), 1);
    }
}
